#!/usr/bin/env python3
import apt
import sys

c = apt.cache.Cache()
c.open()

upgradable_count = 0
for p in c:
    if p.is_upgradable:
        upgradeable_count += 1

print(upgradable_count)
sys.exit(0 if upgradable_count == 0 else 1)
