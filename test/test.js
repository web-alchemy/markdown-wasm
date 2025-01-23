import fs from 'node:fs'
import test from 'node:test'
import assert from 'node:assert/strict'
import { parse, Flags } from '../pkg/markdown_wasm.js'

const readFixtureFile = (fileName) => {
  const file = new URL('./fixtures/' + fileName, import.meta.url)
  return fs.readFileSync(file, 'utf-8')
}

test('should render paragraph', () => {
  const source = readFixtureFile('paragraph.md')
  const html = parse(source)
  assert.equal(html, `<p>${source}</p>\n`)
})

test('should render heading', () => {
  const source = readFixtureFile('headings.md')
  const expected = `<h1>title 1</h1>
<h2>title 2</h2>
<h3>title 3</h3>
<h4>title 4</h4>
<h5>title 5</h5>
<h6>title 6</h6>
`
  const html = parse(source)
  assert.equal(html, expected)
})

test('should render unordered list', () => {
  const source = readFixtureFile('ul.md')
  const expected = `<ul>
<li>1</li>
<li>2</li>
<li>3</li>
</ul>
`;
  const html = parse(source)
  assert.equal(html, expected)
})

test('should render ordered list', () => {
  const source = readFixtureFile('ol.md')
  const expected = `<ol>
<li>one</li>
<li>two</li>
<li>three</li>
</ol>
`;
  const html = parse(source)
  assert.equal(html, expected)
})

test('should render image', () => {
  const altText = `This is alt text`;
  const titleText = `This is title text`
  const sourceUrl = `/assets/images/san-juan-mountains.jpg`
  const source = `![${altText}](${sourceUrl} "${titleText}")`
  const expected = `<p><img src="${sourceUrl}" alt="${altText}" title="${titleText}" /></p>\n`
  const html = parse(source)
  assert.equal(html, expected)
})

test('should render html', () => {
  const source = `<iframe src="https://example.domain/index.html" border="0">`
  const html = parse(source)
  assert.equal(html, source)
})

test('should render utf-8 and emoji', () => {
  const source = `Hello 👋, λ 😀 !!!`
  const expected = `<p>${source}</p>\n`
  const html = parse(source)
  assert.equal(html, expected)
})

test('should render heading with attributes', () => {
  const source = readFixtureFile('flags.md')
  const FLAGS = new Flags()
  const flags = FLAGS.ENABLE_HEADING_ATTRIBUTES | FLAGS.ENABLE_TASKLISTS
  const html = parse(source, flags)
  const expected = `<h1 id="id1" class="class">Example 1</h1>
<ul>
<li><input disabled="" type="checkbox" checked=""/>
check 1</li>
<li><input disabled="" type="checkbox" checked=""/>
check 2</li>
<li><input disabled="" type="checkbox"/>
check 3</li>
</ul>
`
  assert.equal(html, expected)
})