# WebAssembly build of Rust lib [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark) for Node.js

## Installation

```shell
npm install @web-alchemy/markdown-wasm
```

## Usage example:

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

## Development

- Install [Node.js](https://nodejs.org/)
- Install dev dependencies - `npm ci`.
- Modify source code in `src` folder and write tests in `test` folder.
- Create build - `node --run build`. `pkg` folder should contains npm package files.
- Run tests - `node --run test`.