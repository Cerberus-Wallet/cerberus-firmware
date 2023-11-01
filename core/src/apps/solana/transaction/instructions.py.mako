# generated from instructions.py.mako
# do not edit manually!
<%def name="getProgramId(program)">${"_".join(program["name"].upper().split(" ") + ["ID"])}</%def>\
<%def name="getInstructionIdText(program, instruction)">${"_".join([getProgramId(program)] + ["INS"] + instruction["name"].upper().split(" "))}</%def>\
<%def name="getClassName(program, instruction)">${program["name"].replace(" ", "")}${instruction["name"].replace(" ", "")}Instruction</%def>\
<%def name="getReferenceName(reference)">${"_".join(reference["name"].lower().split(" "))}</%def>\
<%def name="getReferenceOptionalType(reference)">\
% if reference["optional"]:
 | None\
% endif
</%def>\
<%def name="getReferenceOptionalTemplate(reference)">\
% if reference["optional"]:
, True\
% else:
, False\
% endif
</%def>\
<%def name="getPythonType(type)">\
% if type in ("u32", "u64", "i32", "i64", "timestamp", "lamports", "token_amount"):
int\
% elif type in ("pubKey", "authority"):
Account\
% elif type in ("string", "memo"):
str\
% else:
int\
% endif
</%def>\
from typing import TYPE_CHECKING

from trezor.wire import DataError

from apps.common.readers import read_uint32_le, read_uint64_le

from ..types import AccountTemplate, InstructionIdFormat, PropertyTemplate, UIProperty
from ..ui.format import (
    format_int,
    format_lamports,
    format_pubkey,
    format_identity,
    format_token_amount,
    format_unix_timestamp,
)
from .instruction import Instruction
from .parse import (
    parse_byte,
    parse_memo,
    parse_optional_pubkey,
    parse_pubkey,
    parse_string,
)

if TYPE_CHECKING:
    from typing import Any, Type

    from ..types import Account

% for program in programs["programs"]:
${getProgramId(program)} = "${program["id"]}"
% endfor

% for program in programs["programs"]:
    % for instruction in program["instructions"]:
${getInstructionIdText(program, instruction)} = ${instruction["id"]}
    % endfor
% endfor

def __getattr__(name: str) -> Type[Instruction]:
    def get_id(name: str) -> tuple[str, int]:
    %for program in programs["programs"]:
        %for instruction in program["instructions"]:
        if name == "${getClassName(program, instruction)}":
            return ("${program["id"]}", ${instruction["id"]})
        %endfor
    %endfor
        raise AttributeError # Unknown instruction

    id = get_id(name)

    class FakeClass(Instruction):
        @classmethod
        def is_type_of(cls, ins: Any):
            return ins.program_id == id[0] and ins.instruction_id == id[1]

    return FakeClass


if TYPE_CHECKING:

% for program in programs["programs"]:
    ## generates classes for instructions
    % for instruction in program["instructions"]:
    class ${getClassName(program, instruction)}(Instruction):
        ## generates properties for instruction parameters
        % for parameter in instruction["parameters"]:
        ${parameter["name"]}: ${getPythonType(parameter["type"])}
        % endfor

        ## generates properties for reference accounts
        % for reference in instruction["references"]:
        ${getReferenceName(reference)}: Account${getReferenceOptionalType(reference)}
        % endfor
    % endfor
% endfor

def get_instruction_id_length(program_id: str) -> InstructionIdFormat:
% for program in programs["programs"]:
    if program_id == ${getProgramId(program)}:
        return InstructionIdFormat(${program["instruction_id_format"]["length"]}, ${program["instruction_id_format"]["is_included_if_zero"]})
% endfor

    return InstructionIdFormat(0, False)


% for _, type in programs["types"].items():
    % if "is_enum" in type and type["is_enum"]:
def ${type["format"]}(_: Instruction, value: int) -> str:
    % for variant in type["fields"]:
    if value == ${variant["value"]}:
        return "${variant["name"]}"
    % endfor
    raise DataError("Unknown value")
    % endif
% endfor

<%def name="getOptionalString(obj, string)">\
% if string in obj:
"${obj[string]}"\
%else:
None\
% endif
</%def>\

def get_instruction(
    program_id: str, instruction_id: int, instruction_accounts: list[Account], instruction_data: bytes
) -> Instruction:
% for program in programs["programs"]:
% if len(program["instructions"]) > 0:
    if program_id == ${getProgramId(program)}:
    % for instruction in program["instructions"]:
        if instruction_id == ${getInstructionIdText(program, instruction)}:
            return Instruction(
                instruction_data,
                program_id,
                instruction_accounts,
                ${getInstructionIdText(program, instruction)},
                [
                % for parameter in instruction["parameters"]:
                    PropertyTemplate(
                        "${parameter["name"]}",
                        ${parameter["type"] == "authority"},
                        ${programs["types"][parameter["type"]]["parse"]},
                        ${programs["types"][parameter["type"]]["format"]},
                    ),
                % endfor
                ],
                [
                % for reference in instruction["references"]:
                    AccountTemplate(
                        "${reference["name"]}",
                        ${reference["is_authority"]},
                        ${reference["optional"]},
                    ),
                % endfor
                ],
                [
                % for ui_property in instruction["ui_properties"]:
                    UIProperty(
                        ${getOptionalString(ui_property, "parameter")},
                        ${getOptionalString(ui_property, "account")},
                        "${ui_property["display_name"]}",
                        ${ui_property["is_authority"] if "is_authority" in ui_property else False},
                    ),
                % endfor
                ],
                "${program["name"]}: ${instruction["name"]}",
                True,
                True,
                ${instruction["is_multisig"]},
                ${getOptionalString(instruction, "is_deprecated_warning")},
            )
    % endfor
        return Instruction(
            instruction_data,
            program_id,
            instruction_accounts,
            instruction_id,
            [],
            [],
            [],
            "${program["name"]}",
            True,
            False,
            False
        )
% endif
% endfor
    return Instruction(
        instruction_data,
        program_id,
        instruction_accounts,
        0,
        [],
        [],
        [],
        "Unsupported program",
        False,
        False,
        False
    )

