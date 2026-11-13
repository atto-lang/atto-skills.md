#!/usr/bin/env python3
"""Dependency-free, fail-closed structural validation for Agent Skills docs."""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SKILLS = ROOT / "skills"
NAME = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
LINK = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
errors = []
skills = sorted(SKILLS.glob("*/SKILL.md"))

if not skills:
    errors.append("No skills/*/SKILL.md files found")

def check_links(text, document):
    # Only validate local Markdown links; https/mailto/anchors are external.
    for target in LINK.findall(text):
        path = target.split("#", 1)[0].split("?", 1)[0]
        if not path or "://" in path or path.startswith(("mailto:", "/")):
            continue
        resolved = (document.parent / path).resolve()
        if not resolved.is_relative_to(ROOT.resolve()) or not resolved.is_file():
            errors.append(f"{document.relative_to(ROOT)}: invalid local link {target}")

for path in skills:
    body = path.read_text(encoding="utf-8")
    match = re.match(r"\A---\r?\n(.*?)\r?\n---\r?\n(.*)\Z", body, re.S)
    if not match:
        errors.append(f"{path.relative_to(ROOT)}: missing YAML frontmatter")
        continue
    header, instructions = match.groups()
    entries = re.findall(r"^([a-z][a-z0-9-]*):[ \t]*(.+)$", header, re.M)
    metadata = dict(entries)
    name, description = metadata.get("name", ""), metadata.get("description", "")
    if len(entries) != len(metadata):
        errors.append(f"{path.relative_to(ROOT)}: duplicate YAML frontmatter key")
    if not NAME.fullmatch(name) or len(name) > 64 or name != path.parent.name:
        errors.append(f"{path.relative_to(ROOT)}: invalid/mismatched skill name")
    if not description or len(description) > 1024:
        errors.append(f"{path.relative_to(ROOT)}: invalid description")
    if not any(s in description.lower() for s in ("use when", "use for")):
        errors.append(f"{path.relative_to(ROOT)}: description needs activation context")
    if len(instructions.strip()) < 100 or not instructions.lstrip().startswith("# "):
        errors.append(f"{path.relative_to(ROOT)}: missing substantive skill instructions")
    if instructions.count("~~~") % 2:
        errors.append(f"{path.relative_to(ROOT)}: unclosed fenced code block")
    check_links(body, path)

for rel in ("README.md", "AGENTS.md", "docs/FEATURE_STATUS.md"):
    path = ROOT / rel
    if not path.is_file():
        errors.append(f"Missing {rel}")
    else:
        check_links(path.read_text(encoding="utf-8"), path)

catalog = (ROOT / "README.md").read_text(encoding="utf-8") if (ROOT / "README.md").exists() else ""
for path in skills:
    if f"skills/{path.parent.name}/SKILL.md" not in catalog:
        errors.append(f"README.md: not indexed: {path.parent.name}")

if errors:
    print("Skill validation failed:")
    for error in errors:
        print(" -", error)
    sys.exit(1)
print(f"Validated {len(skills)} Agent Skills and local documentation links.")
