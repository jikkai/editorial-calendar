# editorial-calendar

> Github-contribution-like calendar for editorial content

```sh
npm run build-release
```


```javascript
import { createRequire } from 'node:module'
const require = createRequire(import.meta.url)

const addon = require('./index.node')

addon.draw(JSON.stringify([
  5, 1, 2, 3, 4, 5, 0,
  // ...
]))
```
