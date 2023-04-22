@nearBindgen
export class PageData {
  title: string;
  description: string;
  links: Link[] = [];
}

@nearBindgen
export class Link {
  type: LinkType;
  text: string;
  path: string;
}

@nearBindgen
export enum LinkType {
  MEDIUM = 0,
  TWITTER = 1,
  TELEGRAM = 2,
  GITHUB = 3,
}

export function skeleton(head: string, inner: string): string {
  return `<html lang='en'>
    <head>
      <meta charset='utf-8' />
      <meta name='viewport' content='width=device-width, initial-scale=1' />
      <link rel='stylesheet' href='css/styles.css' />
      <link rel='icon' type='image/png' href='images/avatar.png' />

      ${head}
    </head>
    <body>

    ${inner}

    </body>
  </html>`;
}

export function body(content: string): string {
  return `<div class='container'>
    <div class='row'>
      <div class='column' style='margin-top: 10%'>
      ${content}
      </div>
    </div>
  </div>`;
}

export function meta(title: string, description: string): string {
  return `<title>${title}</title>
    <meta name="description" content="${description}">
    <meta name="author" content="${title}">`;
}

function getTwitterButton(text: string, path: string): string {
    return `<a
    class='button button-twit'
    href='https://twitter.com/${path}'
    target='_blank'
    rel='noopener'
  >
    <img
      class='icon'
      src='images/icons/twitter.svg'
      alt='Twitter Logo'
    />${text}</a>
  <br />`
}

function getGithubButton(text: string, path: string): string {
    return `<a
       class='button button-github'
       href='https://github.com/${path}'
       target='_blank'
       rel='noopener'
     >
       <img
         class='icon'
         src='images/icons/github.svg'
         alt='GitHub Logo'
       />
       ${text}</a>
     <br />`
}

function getMediumButton(text: string, path: string): string {
    return `<a
    class='button button-medium'
    href='https://medium.com/${path}'
    target='_blank'
    rel='noopener'
  >
    <img
      class='icon'
      src='images/icons/medium.svg'
      alt='Medium Logo'
    />${text}</a>
  <br />`;
}

function getTelegramButton(text: string, path: string): string {
    return `<a
    class='button button-telegram'
    href='https://t.me/${path}'
    target='_blank'
    rel='noopener'
  >
    <img
      class='icon'
      src='images/icons/telegram.svg'
      alt='Telegram Logo'
    />${text}</a>
  <br />`;
}

function buildLinks(links: Link[]): string[] {
  return links.map<string>(function(link: Link): string {
    switch(link.type) {
      case LinkType.TWITTER: return getTwitterButton(link.text, link.path);
      case LinkType.GITHUB: return getGithubButton(link.text, link.path);
      case LinkType.TELEGRAM: return getTelegramButton(link.text, link.path);
      case LinkType.MEDIUM: return getMediumButton(link.text, link.path);
      default: throw new Error(`'Unsupported link type`);
    }
  });
}

export function content(data: PageData): string {
  const content = [`<img
  src='images/avatar.png'
  class='avatar'
  srcset='images/avatar@2x.png 2x'
  alt='LittleLink Logo'
/>
<h1>${data.title}</h1>
<p>${data.description}</p>`];


  const links = buildLinks(data.links);

  const pieces = content.concat(links);

  return pieces.join("");
}
