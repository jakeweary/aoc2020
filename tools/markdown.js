// I used this to convert puzzle descriptions to markdown
// the code supposed to be pasted into the browser console

const mdEl = (el, ul) => {
  switch (el.tagName) {
    case 'BR':   return '\n'
    case 'H2':   return `\n\n# ${el.textContent.replace(/-+ | -+/g, '')}\n\n`
    case 'P':    return `\n\n${mdNodes(el.childNodes, ul)}\n\n`
    case 'UL':   return mdNodes(el.childNodes, ul + 1)
    case 'LI':   return `\n${'  '.repeat(ul)}- ${mdNodes(el.childNodes, ul)}`
    case 'PRE':  return `\n\n\`\`\`\n${el.textContent}\`\`\`\n\n`
    case 'CODE': return `\`${el.textContent}\``
    case 'EM':   return `**${mdNodes(el.childNodes, ul)}**`
    case 'A':    return `[${mdNodes(el.childNodes, ul)}](${el.href})`
    default:     return mdNodes(el.childNodes, ul)
  }
}

const mdNode = (node, ul) => {
  switch (node.nodeType) {
    case Node.ELEMENT_NODE: return mdEl(node, ul)
    case Node.TEXT_NODE:    return node.data.replace(/\n/g, '')
  }
}

const mdNodes = (nodes, ul = -1) =>
  [...nodes].map(n => mdNode(n, ul)).join('')

const md = mdNodes($$('article'))
  .replace(/\n\s*\n/g, '\n\n')
  .replace(/\*\*`(.+?)`\*\*/g, '`$1`')
  .trim()

copy(md)
