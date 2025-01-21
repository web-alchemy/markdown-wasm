# WebAssembly build of Rust lib [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark) for Node.js

Example:

```javascript
import { parse } from '@web-alchemy/markdown-wasm';

const markdownSource = `# title
  Some text
  
  ![some image alt](image.png)
`;

const html = parse(makdownSource);

console.log(html)
/*
<h1>title</h1>
  <p>Some text</p>
  <p><img src="image.png" alt="some image alt" /></p>
*/
```