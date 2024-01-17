import json
from pathlib import Path
from typing import Any, Iterable

from trezorlib import device, translations
from trezorlib.debuglink import TrezorClientDebugLink as Client

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent

TRANSLATIONS = ROOT / "core" / "embed" / "rust" / "src" / "ui" / "translations"
FONTS_DIR = TRANSLATIONS / "fonts"
ORDER_FILE = TRANSLATIONS / "order.json"

LANGUAGES = ["cs", "de", "en", "es", "fr"]


def set_language(client: Client, lang: str):
    if lang.startswith("en"):
        language_data = b""
    else:
        assert lang in LANGUAGES
        language_data = translations.blob_from_file(
            get_lang_json(lang), client.features.model or ""
        )
    with client:
        device.change_language(client, language_data)


def get_lang_json(lang: str) -> Path:
    assert lang in LANGUAGES
    return TRANSLATIONS / f"{lang}.json"


def _get_all_language_data() -> list[dict[str, dict[str, str]]]:
    return [_get_language_data(language) for language in LANGUAGES]


def _get_language_data(lang: str) -> dict[str, dict[str, str]]:
    file = get_lang_json(lang)
    return json.loads(file.read_text())["translations"]


all_language_data = _get_all_language_data()


def _resolve_path_to_texts(
    path: str, template: Iterable[Any] = (), lower: bool = True
) -> list[str]:
    texts: list[str] = []
    lookups = path.split(".")
    for language_data in all_language_data:
        data: dict[str, Any] | str = language_data
        for lookup in lookups:
            assert isinstance(data, dict), f"{lookup} is not a dict"
            data = data[lookup]
        assert isinstance(data, str), f"{path} is not a string"
        if template:
            data = data.format(*template)
        texts.append(data)

    if lower:
        texts = [t.lower() for t in texts]
    return texts


def assert_equals(text: str, path: str, template: Iterable[Any] = ()) -> None:
    # TODO: we can directly pass in the current device language
    texts = _resolve_path_to_texts(path, template)
    assert text.lower() in texts, f"{text} not found in {texts}"


def assert_in(text: str, path: str, template: Iterable[Any] = ()) -> None:
    texts = _resolve_path_to_texts(path, template)
    for t in texts:
        if t in text.lower():
            return
    assert False, f"{text} not found in {texts}"


def assert_startswith(text: str, path: str, template: Iterable[Any] = ()) -> None:
    texts = _resolve_path_to_texts(path, template)
    for t in texts:
        if text.lower().startswith(t):
            return
    assert False, f"{text} not found in {texts}"


def assert_template(text: str, template_path: str) -> None:
    templates = _resolve_path_to_texts(template_path)
    for t in templates:
        # Checking at least the first part
        first_part = t.split("{")[0]
        if text.lower().startswith(first_part):
            return
    assert False, f"{text} not found in {templates}"


def translate(path: str, template: Iterable[Any] = (), lower: bool = False) -> list[str]:
    # Do not converting to lowercase, we want the exact value
    return _resolve_path_to_texts(path, template, lower=lower)
