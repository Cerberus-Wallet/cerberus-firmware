#!/usr/bin/env python3

import datetime
from pathlib import Path
import re
import subprocess

import click

LINK_RE = re.compile(r"\[#(\d+)\]")
ISSUE_URL = "https://github.com/cerberus/cerberus-firmware/pull/{issue}"

VERSION_HEADER_RE = re.compile(r"## \[([.0-9]+)\]")
DIFF_LINK = "[{new}]: https://github.com/cerberus/cerberus-firmware/compare/{tag_prefix}{old}...{tag_prefix}{new}\n"

MODELS_RE = re.compile(r"\[([A-Z0-9]{4})(,[A-Z0-9]{4})*\][ ]?")
INTERNAL_MODELS = ("T2T1", "T2B1", "D001")
INTERNAL_MODELS_SKIP = ("D001",)


def linkify_changelog(changelog_file, only_check=False):
    links = {}
    orig_links = {}
    result_lines = []

    with open(changelog_file, "r+") as changelog:
        for line in changelog:
            m = LINK_RE.match(line)
            if m:  # line *starts with* issue identifier
                # keep existing links as-is
                orig_links[int(m[1])] = line.replace(m[0] + ": ", "").strip()
            else:
                for issue in LINK_RE.findall(line):
                    links[int(issue)] = ISSUE_URL.format(issue=issue)
                result_lines.append(line)

        if only_check:
            missing_links = set(links.keys()) - set(orig_links.keys())
            if missing_links:
                click.echo(f"missing links: {missing_links}")
                return False
            else:
                return True

        links.update(orig_links)

        changelog.seek(0)
        changelog.truncate(0)
        for line in result_lines:
            changelog.write(line)
        for marker, url in sorted(links.items()):
            changelog.write(f"[#{marker}]: {url}\n")

    return True


def linkify_gh_diff(changelog_file, tag_prefix):
    linkified = False
    versions = []
    result_lines = []

    with open(changelog_file, "r+") as changelog:
        for line in changelog:
            m = VERSION_HEADER_RE.match(line)
            if m:
                versions.append(m[1])
            result_lines.append(line)

        changelog.seek(0)
        changelog.truncate(0)
        for line in result_lines:
            changelog.write(line)
            if not linkified and VERSION_HEADER_RE.match(line):
                changelog.write(
                    DIFF_LINK.format(
                        tag_prefix=tag_prefix, new=versions[0], old=versions[1]
                    )
                )
                linkified = True


def current_date(project):
    parts = project.parts
    today = datetime.datetime.now()

    if (
        parts[-3:] == ("core", "embed", "boardloader")
        or parts[-3:] == ("core", "embed", "bootloader")
        or parts[-3:] == ("core", "embed", "bootloader_ci")
        or parts[-2:] == ("legacy", "bootloader")
        or parts[-2:] == ("legacy", "intermediate_fw")
    ):
        return today.strftime("%B %Y")
    elif parts[-1] == "python":
        return today.strftime("%Y-%m-%d")
    else:
        daysuffix = {1: "st", 2: "nd", 3: "rd"}.get(today.day % 10, "th")
        return today.strftime(f"%-d{daysuffix} %B %Y")


def filter_changelog(changelog_file, internal_name):
    def filter_line(line):
        m = MODELS_RE.search(line)
        if not m:
            return line
        if internal_name in m[0]:
            return MODELS_RE.sub("", line, count=1)
        else:
            return None

    destination_file = changelog_file.with_suffix(f".{internal_name}.md")
    with open(changelog_file, "r") as changelog, open(destination_file, "w") as destination:
        for line in changelog:
            res = filter_line(line)
            if res is not None:
                destination.write(res)


@click.command()
@click.argument(
    "project",
    type=click.Path(exists=True, dir_okay=True, file_okay=False, resolve_path=True),
)
@click.argument(
    "version",
    type=str,
    required=False,
)
@click.option("--date", help="Specify release date (default: today).")
@click.option(
    "--check", is_flag=True, help="Dry run, do not actually create changelog."
)
def cli(project, version, date, check):
    """Generate changelog for given project (core, python, legacy/firmware,
    legacy/bootloader).

    - Run towncrier to assemble changelog from fragments in .changelog.d/.

    - Find all occurences of "[#123]" in text, and add a Markdown link to the
      referenced issue.

    - Tell git to stage changed files.
    """
    project = Path(project)
    changelog = project / "CHANGELOG.md"

    if not changelog.exists():
        raise click.ClickException(f"{changelog} not found")

    if version is None:
        if not check:
            raise click.ClickException("Version argument is required.")
        version = "unreleased"

    if date is None:
        date = current_date(project)

    args = ["towncrier", "build", "--yes", "--version", version, "--date", date]
    if check:
        args.append("--draft")
    subprocess.run(args, cwd=project, check=True)

    if not check:
        linkify_changelog(changelog)

        # python changelog has links to github diffs
        if project.parts[-1] == "python":
            linkify_gh_diff(changelog, tag_prefix="python/v")

        # core changelog for each model
        if project.parts[-1] == "core":
            for internal_name in INTERNAL_MODELS:
                if internal_name in INTERNAL_MODELS_SKIP:
                    continue
                filter_changelog(changelog, internal_name)

        # towncrier calls git add before we do linkification, stage the changes too
        subprocess.run(["git", "add", changelog], check=True)


if __name__ == "__main__":
    cli()
