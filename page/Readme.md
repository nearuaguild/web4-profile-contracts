<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h2 align="center">Near Protocol | Web4 Profile Contract</h2>

  <p align="center">
    A working Web4 Profile Smart Contract, based on <a href="https://littlelink.io/"> open source self-hosted LittleLink</a>
    <br />
    <br />
    <a href="https://github.com/nearuaguild/near-web4-widgets">Widget to create pages</a>
    ·
    <a href="https://github.com/nearuaguild"> Explore other projects</a>
    ·
    <a href="https://github.com/nearuaguild/near-web4-contracts/issues">Report a bug</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

A smart contract built on Near Protocol that allows anyone to deploy their own self-hosted Linktree alternative site that is tied to your `.near` account with the help of [web4](https://github.com/vgrichina/web4)

Data is stored on the Near Protocol chain - no one can change it except you once deployed 

### Built With

- [![Assembly Script][assemblyscript]][assemblyscript-url]
- [near-sdk-as (v3.2.3)](https://github.com/near/near-sdk-as)
- [![Littlelink][littlelink]][littlelink-url]

Please take note that the decision to use AssemblyScript for writing the smart contract was not arbitrary

<style>
  #compiled_wasm_size_table tr:nth-child(2) {
    background-color: #069C56AE;
  }
</style>
<div id="compiled_wasm_size_table">

| **Language** | **SDK** | **Minimal WASM size** |
|:---:|:---:|:---:|
| Rust | [near-sdk-rs](https://github.com/near/near-sdk-rs) | ~150kb |
| AssemblyScript | [near-sdk-as](https://github.com/near/near-sdk-as) | ~25kb |
| JavaScript | [near-sdk-js](https://github.com/near/near-sdk-js) | ~480kb |

</div>

And it comes from the fact that `rust-sdk-as` produces compiled WASM a few times as less as other SDKs

The deployment cost of a smart contract increases in proportion to its size, as more [Gas](https://docs.near.org/concepts/basics/transactions/gas-advanced) is required


---

<!-- GETTING STARTED -->

## Getting Started

💡 _Before starting, please ensure that you have all the necessary installations completed_

- [Near CLI](https://docs.near.org/tools/near-cli#setup)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install)

### Installation

Follow these simple instructions to set up a local development environment

1. Clone the repo
   ```sh
   git clone https://github.com/nearuaguild/web4-profile-contracts.git
   ```
2. Open project folder
  ```sh
  cd web4-profile-contracts/page
  ```
3. Install dependencies
  ```sh
  yarn
  ```
4. Create Web4 subaccount (replace `YOUR_ACCOUNT` with your own account name)
  ```sh
  NODE_ENV=mainnet near create-account web4.YOUR_ACCOUNT.near --masterAccount YOUR_ACCOUNT.near --initialBalance 0.5
  ```
5. Deploy smart contract
  ```sh
  NODE_ENV=mainnet CONTRACT_NAME=web4.YOUR_ACCOUNT.near yarn deploy
  ```
6. Initialize contract with your data & links (replace `YOUR_ACCOUNT` with your own account name)
  ```sh
  NODE_ENV=mainnet near call web4.YOUR_ACCOUNT.near init \
    '{"data":{"title":"Near Ukraine Guild","description":"Fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem", "links":[{
      "type": 0,
      "text": "Medium",
      "path": "@nearuaguild"
    },{
      "type": 1,
      "text": "Twitter",
      "path": "nearuaguild"
    },{
      "type": 2,
      "text": "Telegram",
      "path": "nearprotocoluachannel"
    },{
      "type": 3,
      "text": "GitHub",
      "path": "nearuaguild"
    }]}, "ipfs": "ipfs://bafybeihiw6mwe63nvy6it7sialrr3fgx57jqnfdqlpxg7do6fytbxnzmna"}' --accountId  web4.YOUR_ACCOUNT.near
  ```

You can access your deployed smart contract at https://YOUR_ACCOUNT.near.page

This is hosted web4 gateway provided to all `.near` accounts

For now, it's free, but in the future, it might change

## What's next?

You may want to consider reviewing the factory smart contract that takes care of deploying pages automatically - more information [here](https://github.com/nearuaguild/web4-profile-contracts/tree/main/factory)

## Developed by

![Near Ukrainians Guild cover](../images/cover.png)

**Near Ukrainians Guild** is a fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem

[![Twitter][twitter]][twitter-url]
[![Youtube][youtube]][youtube-url]
[![Telegram Chat][telegram-chat]][telegram-chat-url]
[![Telegram Channel][telegram-channel]][telegram-channel-url]
[![Medium][medium]][medium-url]
[![Github][github]][github-url]

<!-- LICENSE -->

## License

Based on [LittleLink](https://littlelink.io/)

See `LICENSE.txt` for more information

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

<!-- Built with -->

[javascript]: https://img.shields.io/badge/javascript-000000?style=for-the-badge&logo=javascript&logoColor=F7E018
[javascript-url]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
[assemblyscript]: https://img.shields.io/badge/assembly%20script-1B7ACE?style=for-the-badge&logo=assemblyscript&logoColor=white
[assemblyscript-url]: https://www.assemblyscript.org/
[littlelink]: https://img.shields.io/badge/LittleLink-1D84FF?style=for-the-badge
[littlelink-url]: https://littlelink.io/

<!-- Socials -->

[twitter]: https://img.shields.io/badge/news-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white
[youtube]: https://img.shields.io/badge/broadcasting-282828?style=for-the-badge&logo=youtube&logoColor=ff0000
[medium]: https://img.shields.io/badge/articles-202020?style=for-the-badge&logo=medium&logoColor=ffffff
[telegram-chat]: https://img.shields.io/badge/chat-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[telegram-channel]: https://img.shields.io/badge/channel-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[github]: https://img.shields.io/badge/code-000000?style=for-the-badge&logo=github&logoColor=ffffff
[twitter-url]: https://twitter.com/nearuaguild
[youtube-url]: https://www.youtube.com/@nearprotocolukraineguild4064
[medium-url]: https://medium.com/near-protocol-ua
[telegram-chat-url]: https://t.me/nearprotocolua
[telegram-channel-url]: https://t.me/nearprotocoluachannel
[github-url]: https://github.com/nearuaguild