from typing import TYPE_CHECKING

from .. import buttons
from .. import translations as TR
from .common import go_next

if TYPE_CHECKING:
    from trezorlib.debuglink import DebugLink, LayoutContent


def enter_word(
    debug: "DebugLink", word: str, is_slip39: bool = False
) -> "LayoutContent":
    if debug.model == "T":
        typed_word = word[:4]
        for coords in buttons.type_word(typed_word, is_slip39=is_slip39):
            debug.click(coords)

        return debug.click(buttons.CONFIRM_WORD, wait=True)
    elif debug.model == "Safe 3":
        letter_index = 0
        layout = debug.read_layout()

        # Letter choices
        while layout.find_values_by_key("letter_choices"):
            letter = word[letter_index]
            while not layout.get_middle_choice() == letter:
                layout = debug.press_right(wait=True)

            layout = debug.press_middle(wait=True)
            letter_index += 1

        # Word choices
        while not layout.get_middle_choice() == word:
            layout = debug.press_right(wait=True)

        return debug.press_middle(wait=True)
    else:
        raise ValueError("Unknown model")


def confirm_recovery(debug: "DebugLink") -> None:
    layout = debug.wait_layout()
    TR.assert_equals(layout.title(), "recovery__title")
    if debug.model == "T":
        debug.click(buttons.OK, wait=True)
    elif debug.model == "Safe 3":
        debug.press_right(wait=True)
        debug.press_right()


def select_number_of_words(
    debug: "DebugLink", num_of_words: int = 20, wait: bool = True
) -> None:
    if wait:
        debug.wait_layout()
    TR.assert_equals(debug.read_layout().text_content(), "recovery__num_of_words")
    if debug.model == "T":
        # click the number
        word_option_offset = 6
        word_options = (12, 18, 20, 24, 33)
        index = word_option_offset + word_options.index(
            num_of_words
        )  # raises if num of words is invalid
        coords = buttons.grid34(index % 3, index // 3)
        layout = debug.click(coords, wait=True)
    elif debug.model == "Safe 3":
        layout = debug.press_right(wait=True)
        TR.assert_equals(layout.title(), "word_count__title")

        # navigate to the number and confirm it
        word_options = (12, 18, 20, 24, 33)
        index = word_options.index(num_of_words)
        for _ in range(index):
            debug.press_right(wait=True)
        layout = debug.press_middle(wait=True)
    else:
        raise ValueError("Unknown model")

    if num_of_words in (20, 33):
        TR.assert_in(layout.text_content(), "recovery__enter_any_share")
    else:
        TR.assert_in(layout.text_content(), "recovery__enter_backup")


def enter_share(
    debug: "DebugLink", share: str, is_first: bool = True
) -> "LayoutContent":
    TR.assert_in(debug.read_layout().title(), "recovery__title_recover")
    if debug.model == "Safe 3":
        layout = debug.press_right(wait=True)
        if is_first:
            # Word entering info
            debug.press_right()
            layout = debug.press_right(wait=True)
    else:
        layout = debug.click(buttons.OK, wait=True)

    assert "MnemonicKeyboard" in layout.all_components()

    for word in share.split(" "):
        layout = enter_word(debug, word, is_slip39=True)

    return layout


def enter_shares(debug: "DebugLink", shares: list[str]) -> None:
    TR.assert_in(debug.read_layout().text_content(), "recovery__enter_any_share")
    for index, share in enumerate(shares):
        enter_share(debug, share, is_first=index == 0)
        if index < len(shares) - 1:
            TR.assert_in(
                debug.read_layout().text_content(),
                "recovery__x_of_y_entered_template",
                template=(index + 1, len(shares)),
            )

    TR.assert_in(debug.read_layout().text_content(), "recovery__wallet_recovered")


def enter_seed(debug: "DebugLink", seed_words: list[str]) -> None:
    TR.assert_in(debug.read_layout().text_content(), "recovery__enter_backup")
    if debug.model == "T":
        debug.click(buttons.OK, wait=True)
    elif debug.model == "Safe 3":
        debug.press_right(wait=True)
        TR.assert_equals(debug.read_layout().title(), "recovery__title_recover")
        debug.press_right()

        debug.press_right(wait=True)

    assert "MnemonicKeyboard" in debug.read_layout().all_components()
    for word in seed_words:
        enter_word(debug, word, is_slip39=False)

    TR.assert_in(debug.read_layout().text_content(), "recovery__wallet_recovered")


def finalize(debug: "DebugLink") -> None:
    layout = go_next(debug, wait=True)
    assert layout is not None
    assert layout.main_component() == "Homescreen"
