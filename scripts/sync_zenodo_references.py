#!/usr/bin/env python3
"""Synchronize .zenodo.json references from refs.bib and manual/refs.bib."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BIB_FILES = [ROOT / "refs.bib", ROOT / "manual" / "refs.bib"]
ZENODO_FILE = ROOT / ".zenodo.json"


def iter_bib_entries(text: str):
    i = 0
    n = len(text)
    while i < n:
        at = text.find("@", i)
        if at == -1:
            return
        brace = text.find("{", at)
        if brace == -1:
            return
        kind = text[at + 1 : brace].strip()
        comma = text.find(",", brace)
        if comma == -1:
            return
        key = text[brace + 1 : comma].strip()
        depth = 0
        j = brace
        while j < n:
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    yield kind, key, text[comma + 1 : j]
                    i = j + 1
                    break
            j += 1
        else:
            return


def parse_fields(body: str) -> dict[str, str]:
    fields: dict[str, str] = {}
    i = 0
    n = len(body)
    while i < n:
        while i < n and (body[i].isspace() or body[i] == ","):
            i += 1
        start = i
        while i < n and (body[i].isalnum() or body[i] in "_-:"):
            i += 1
        if i == start:
            i += 1
            continue
        name = body[start:i].strip().lower()
        while i < n and body[i].isspace():
            i += 1
        if i >= n or body[i] != "=":
            continue
        i += 1
        while i < n and body[i].isspace():
            i += 1
        if i < n and body[i] == "{":
            depth = 0
            start_val = i + 1
            while i < n:
                if body[i] == "{":
                    depth += 1
                elif body[i] == "}":
                    depth -= 1
                    if depth == 0:
                        fields[name] = body[start_val:i].strip()
                        i += 1
                        break
                i += 1
        elif i < n and body[i] == '"':
            i += 1
            start_val = i
            while i < n and body[i] != '"':
                i += 1
            fields[name] = body[start_val:i].strip()
            i += 1
        else:
            start_val = i
            while i < n and body[i] not in ",\n":
                i += 1
            fields[name] = body[start_val:i].strip()
    return fields


def latex_to_text(value: str) -> str:
    s = value
    s = s.replace(r"{\"a}", "ä").replace(r"{\"o}", "ö").replace(r"{\"u}", "ü")
    s = s.replace(r"\"a", "ä").replace(r"\"o", "ö").replace(r"\"u", "ü")
    for cmd in ["emph", "textit", "textbf", "mathrm"]:
        pat = re.compile(r"\\" + cmd + r"\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}")
        while True:
            ns = pat.sub(r"\1", s)
            if ns == s:
                break
            s = ns
    s = re.sub(r"\\url\{([^{}]+)\}", r"\1", s)
    s = s.replace(r"\&", "&")
    s = s.replace("{", "").replace("}", "")
    s = s.replace("--", "–")
    s = re.sub(r"\\([A-Za-z]+)", r"\1", s)
    s = re.sub(r"\s+", " ", s).strip()
    return s


def reference_from_fields(fields: dict[str, str]) -> str | None:
    author = fields.get("author")
    title = fields.get("title")
    year = fields.get("year")
    if not (author and title and year):
        note = fields.get("note")
        return latex_to_text(note) if note else None

    author_text = latex_to_text(author).rstrip(".")
    title_text = latex_to_text(title).rstrip(".")
    venue = (
        fields.get("journal")
        or fields.get("publisher")
        or fields.get("institution")
        or fields.get("howpublished")
    )
    doi = fields.get("doi")
    url = fields.get("url")

    rest = title_text
    if venue:
        rest += f". {latex_to_text(venue).rstrip('.')}"
    rest += f", {latex_to_text(year).rstrip('.')}"
    if doi:
        rest += f". https://doi.org/{latex_to_text(doi).rstrip('.')}"
    elif url:
        rest += f". {latex_to_text(url).rstrip('.')}"
    return f"{author_text}. {rest}."


def generated_references() -> list[str]:
    seen: set[str] = set()
    refs: list[str] = []
    for bib in BIB_FILES:
        if not bib.exists():
            continue
        for _kind, key, body in iter_bib_entries(bib.read_text(encoding="utf-8")):
            if key in seen:
                continue
            fields = parse_fields(body)
            ref = reference_from_fields(fields)
            if not ref:
                continue
            seen.add(key)
            refs.append(ref)
    return refs


def update_zenodo(check: bool = False) -> int:
    data = json.loads(ZENODO_FILE.read_text(encoding="utf-8"))
    expected = generated_references()
    current = data.get("references", [])
    if current == expected:
        return 0
    if check:
        print(
            ".zenodo.json references are not synchronized with refs.bib/manual/refs.bib",
            file=sys.stderr,
        )
        print(
            f"expected {len(expected)} references, found {len(current)}",
            file=sys.stderr,
        )
        return 1
    data["references"] = expected
    ZENODO_FILE.write_text(
        json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--check", action="store_true", help="verify without modifying .zenodo.json"
    )
    args = ap.parse_args()
    return update_zenodo(check=args.check)


if __name__ == "__main__":
    raise SystemExit(main())
