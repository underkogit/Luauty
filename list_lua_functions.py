import re
import sys
from pathlib import Path

CREATE_FUNC_RE = re.compile(r'let\s+(\w+)\s*=\s*lua\.create_function\(\s*\|lua,\s*(.*?)\s*\|')
SET_RE = re.compile(r'globals\.set\(\s*"([^"]+)"\s*,\s*(\w+)\s*\)')

def main(root: str) -> None:
    root_path = Path(root)
    functions = []

    for path in root_path.rglob("*.rs"):
        text = path.read_text(encoding="utf-8", errors="ignore")
        created = {}
        for var, params in CREATE_FUNC_RE.findall(text):
            created[var] = params.strip()
        for name, var in SET_RE.findall(text):
            if var in created:
                functions.append((name, created[var]))

    for name, params in sorted(functions, key=lambda x: x[0]):
        print(f"{name}({params})")

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")
