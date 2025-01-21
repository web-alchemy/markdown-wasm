# WebAssembly build of Rust lib [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark) for Node.js

Example:

```javascript
import { parse } from '@web-alchemy/markdown-wasm';

const makdownSource = `# title
  Some text
  
  ![some image alt](image.png)
`;

const html = parse(makdownSource)
```