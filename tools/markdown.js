// I used this to convert puzzle descriptions to markdown
// the code supposed to be pasted into the browser console

const mdUl = (n, ul) => {
  const md = mdNodes(n.children, ul + 1).trimEnd()
  return ul ? `\n${md}` : `${md}\n\n`
}

const mdEl = (n, ul) => {
  switch (n.tagName) {
    case 'BR': return '\n'
    case 'UL': return mdUl(n, ul)
    case 'LI': return `${'  '.repeat(ul - 1)}- ${mdNodes(n.childNodes, ul)}\n`
    case 'H2': return `# ${n.textContent.replace(/-+ | -+/g, '')}\n\n`
    case 'PRE': return `\`\`\`\n${n.textContent}\`\`\`\n\n`
    case 'CODE': return `\`${n.textContent}\``
    case 'EM': return `**${mdNodes(n.childNodes, ul)}**`
    case 'A': return `[${mdNodes(n.childNodes, ul)}](${n.href})`
    case 'P': return `${mdNodes(n.childNodes, ul)}\n\n`
    default: return mdNodes(n.childNodes, ul)
  }
}

const mdNode = (n, ul) => {
  switch (n.nodeType) {
    case Node.ELEMENT_NODE: return mdEl(n, ul)
    case Node.TEXT_NODE: return n.data.replace(/\n/g, '')
  }
}

const mdNodes = (nodes, ul = 0) =>
  [...nodes].map(n => mdNode(n, ul)).join('')

const md = mdNodes($$('article'))
  .replace(/\*\*`(.+?)`\*\*/g, '`$1`')
  .trim()

copy(md)
