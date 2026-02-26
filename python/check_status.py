"""Check builder fee approval status."""

import json
from client import get_approval, address

res = get_approval(address)
print(json.dumps(res, indent=2))
