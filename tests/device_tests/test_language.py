# This file is part of the Trezor project.
#
# Copyright (C) 2012-2019 SatoshiLabs and contributors
#
# This library is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License version 3
# as published by the Free Software Foundation.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the License along with this library.
# If not, see <https://www.gnu.org/licenses/lgpl-3.0.html>.

import json
from contextlib import contextmanager
from copy import deepcopy
from pathlib import Path
from typing import Any, Dict, Generator

import pytest

from trezorlib import debuglink, device, exceptions, messages, translations
from trezorlib.debuglink import TrezorClientDebugLink as Client

from ..translations import FONTS_DIR, LANGUAGES, ORDER_FILE, get_lang_json, set_language

pytestmark = pytest.mark.skip_t1


MAX_DATA_LENGTH = {"T": 48 * 1024, "Safe 3": 32 * 1024}


def _read_confirm_word(file: Path) -> str:
    content = json.loads(file.read_text())
    return content["translations"]["words__confirm"]


def get_confirm(lang: str) -> str:
    assert lang in LANGUAGES
    return _read_confirm_word(get_lang_json(lang))


@contextmanager
def _set_english_return_back(client: Client) -> Generator[Client, None, None]:
    lang_before = client.features.language or ""
    try:
        set_language(client, "en")
        yield client
    finally:
        set_language(client, lang_before)


def _get_data_from_dict(data: Dict[str, Any], client: Client) -> bytes:
    return translations.blob_from_dict(
        data,
        font_dir=FONTS_DIR,
        order_json_file=ORDER_FILE,
        model=client.features.model or "",
    )


def _check_ping_screen_texts(client: Client, title: str, right_button: str) -> None:
    def ping_input_flow(client: Client, title: str, right_button: str):
        yield
        layout = client.debug.wait_layout()
        assert layout.title() == title.upper()
        assert layout.button_contents()[-1] == right_button.upper()
        client.debug.press_yes()

    # TT does not have a right button text (but a green OK tick)
    if client.features.model == "T":
        right_button = "-"

    with client:
        client.watch_layout(True)
        client.set_input_flow(ping_input_flow(client, title, right_button))
        ping = client.call(messages.Ping(message="ahoj!", button_protection=True))
        assert ping == messages.Success(message="ahoj!")


def test_change_language_errors(client: Client):
    with _set_english_return_back(client) as client:
        assert client.features.language == "en-US"

        # Translations too short
        # Sending less data than the header length
        with pytest.raises(
            exceptions.TrezorFailure, match="Translations too short"
        ), client:
            bad_data = (translations.HEADER_LEN - 1) * b"a"
            device.change_language(client, language_data=bad_data)
        assert client.features.language == "en-US"

        # Translations too long
        # Sending more than allowed by the flash capacity
        max_length = MAX_DATA_LENGTH[client.features.model]
        with pytest.raises(
            exceptions.TrezorFailure, match="Translations too long"
        ), client:
            bad_data = (max_length + 1) * b"a"
            device.change_language(client, language_data=bad_data)
        assert client.features.language == "en-US"

        # Invalid header data length
        # Sending more data than advertised in the header
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid header data length"
        ), client:
            good_data = translations.blob_from_file(
                get_lang_json("cs"), client.features.model or ""
            )
            bad_data = good_data + b"abcd"
            device.change_language(client, language_data=bad_data)
        assert client.features.language == "en-US"

        # Invalid header magic
        # Does not match the expected magic
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid header magic"
        ), client:
            good_data = translations.blob_from_file(
                get_lang_json("cs"), client.features.model or ""
            )
            bad_data = 4 * b"a" + good_data[4:]
            device.change_language(client, language_data=bad_data)
        assert client.features.language == "en-US"

        # Invalid header data
        # Putting non-zero bytes where zero is expected
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid header data"
        ), client:
            good_data = translations.blob_from_file(
                get_lang_json("cs"), client.features.model or ""
            )
            pre_sig_pos = translations.HEADER_LEN - translations.SIG_LEN
            bad_data = good_data[: pre_sig_pos - 4] + 4 * b"a" + good_data[pre_sig_pos:]
            device.change_language(
                client,
                language_data=bad_data,
            )
        assert client.features.language == "en-US"

        # Invalid data hash
        # Changing the data after their hash has been calculated
        with pytest.raises(exceptions.TrezorFailure, match="Invalid data hash"), client:
            good_data = translations.blob_from_file(
                get_lang_json("cs"), client.features.model or ""
            )
            bad_data = good_data[:-8] + 8 * b"a"
            device.change_language(
                client,
                language_data=bad_data,
            )
        assert client.features.language == "en-US"

        # Invalid translations version
        # Change the version to one not matching the current device
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid translations version"
        ), client:
            with open(get_lang_json("cs"), "r") as f:
                data = json.load(f)
            data["header"]["version"] = "3.5.4"
            device.change_language(
                client,
                language_data=_get_data_from_dict(data, client),
            )
        assert client.features.language == "en-US"

        # Invalid header version
        # Version is not a valid semver with integers
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid header version"
        ), client:
            with open(get_lang_json("cs"), "r") as f:
                data = json.load(f)
            data["header"]["version"] = "ABC.XYZ.DEF"
            device.change_language(
                client,
                language_data=_get_data_from_dict(data, client),
            )
        assert client.features.language == "en-US"

        # Invalid translations signature
        # Modifying the signature part of the header
        with pytest.raises(
            exceptions.TrezorFailure, match="Invalid translations signature"
        ), client:
            good_data = translations.blob_from_file(
                get_lang_json("cs"), client.features.model or ""
            )
            bad_data = (
                good_data[: translations.HEADER_LEN - 8]
                + 8 * b"a"
                + good_data[translations.HEADER_LEN :]
            )
            device.change_language(
                client,
                language_data=bad_data,
            )
        assert client.features.language == "en-US"

        _check_ping_screen_texts(client, get_confirm("en"), get_confirm("en"))


def test_full_language_change(client: Client):
    with _set_english_return_back(client) as client:
        assert client.features.language == "en-US"

        # Setting cs language
        set_language(client, "cs")
        assert client.features.language == "cs"
        _check_ping_screen_texts(client, get_confirm("cs"), get_confirm("cs"))

        # Setting fr language
        set_language(client, "fr")
        assert client.features.language == "fr"
        _check_ping_screen_texts(client, get_confirm("fr"), get_confirm("fr"))

        # Setting the default language via empty data
        set_language(client, "en")
        assert client.features.language == "en-US"
        _check_ping_screen_texts(client, get_confirm("en"), get_confirm("en"))


def test_language_stays_after_wipe(client: Client):
    with _set_english_return_back(client) as client:
        assert client.features.language == "en-US"

        _check_ping_screen_texts(client, get_confirm("en"), get_confirm("en"))

        # Setting cs language
        set_language(client, "cs")
        assert client.features.language == "cs"

        _check_ping_screen_texts(client, get_confirm("cs"), get_confirm("cs"))

        # Wipe device
        device.wipe(client)
        assert client.features.language == "cs"

        # Load it again
        debuglink.load_device(
            client,
            mnemonic=" ".join(["all"] * 12),
            pin=None,
            passphrase_protection=False,
            label="test",
        )
        assert client.features.language == "cs"

        _check_ping_screen_texts(client, get_confirm("cs"), get_confirm("cs"))


def test_translations_renders_on_screen(client: Client):
    with open(get_lang_json("cs"), "r") as f:
        czech_data = json.load(f)

    # Setting some values of words__confirm key and checking that in ping screen title
    with _set_english_return_back(client) as client:
        assert client.features.language == "en-US"

        # Normal english
        _check_ping_screen_texts(client, get_confirm("en"), get_confirm("en"))

        # Normal czech
        set_language(client, "cs")
        assert client.features.language == "cs"
        _check_ping_screen_texts(client, get_confirm("cs"), get_confirm("cs"))

        # Modified czech - changed value
        czech_data_copy = deepcopy(czech_data)
        new_czech_confirm = "ABCD"
        czech_data_copy["translations"]["words__confirm"] = new_czech_confirm
        device.change_language(
            client,
            language_data=_get_data_from_dict(czech_data_copy, client),
        )
        _check_ping_screen_texts(client, new_czech_confirm, get_confirm("cs"))

        # Modified czech - key deleted completely, english is shown
        czech_data_copy = deepcopy(czech_data)
        del czech_data_copy["translations"]["words__confirm"]
        device.change_language(
            client, language_data=_get_data_from_dict(czech_data_copy, client)
        )
        _check_ping_screen_texts(client, get_confirm("en"), get_confirm("cs"))
