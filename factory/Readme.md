<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h2 align="center">Near Protocol | Web4 Profiles Factory Contract</h2>

  <p align="center">
    A working Factory Smart Contract that delivers a fast deployment pipeline based on the <a href="https://github.com/nearuaguild/web4-profile-contracts/tree/main/page"> Web4 site Contract</a> 
    <br />
    <br />
    <a href="https://github.com/nearuaguild/web4-profile-widgets">Widget to create pages</a>
    ·
    <a href="https://github.com/nearuaguild"> Explore other projects</a>
    ·
    <a href="https://github.com/nearuaguild/near-web4-contracts/issues">Report a bug</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

A Factory Smart Contract, which takes care of the entire deployment process seamlessly, saving your time

Operates alongside the <a href="https://github.com/nearuaguild/web4-profile-contracts/tree/main/page">Web4 Smart Contract</a> to enhance its deployment speed & convenience

### Built With

- [![Rust][rust]][rust-url]
- [near-sdk-rs (v4.0.0)](https://github.com/near/near-sdk-rs)

---

<!-- GETTING STARTED -->

## Getting Started

💡 _Before starting, please ensure that you have all the necessary installations completed_

- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cargo](https://github.com/rust-lang/cargo#compiling-from-source)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git/)
- [Near CLI](https://docs.near.org/tools/near-cli#setup)

### Installation

Follow these simple instructions to set up a local development environment

1. Clone the repo
  ```sh
   git clone https://github.com/nearuaguild/web4-profile-contracts.git
  ```
2. Open project folder
  ```sh
  cd web4-profile-contracts/factory
  ```
3. Compile Smart Contracts
  ```sh
  ./build.sh
  ```
4. Create Factory subaccount (replace `YOUR_ACCOUNT` with your own account name)
  ```sh
  NODE_ENV=mainnet near create-account page_factory.YOUR_ACCOUNT.near --masterAccount YOUR_ACCOUNT.near --initialBalance 2
  ```
5. Deploy
  ```sh
  NODE_ENV=mainnet near deploy CONTRACT_NAME=page_factory.YOUR_ACCOUNT.near ./target/wasm32-unknown-unknown/release/factory.wasm
  ```
6. Initialize contract
  ```sh
  NODE_ENV=mainnet near call page_factory.YOUR_ACCOUNT.near new '{}' --accountId YOUR_ACCOUNT.near
  ```
7. Set IPFS Gateway url for static files from Web4 Profile Contract
  ```sh
  NODE_ENV=mainnet near call page_factory.YOUR_ACCOUNT.near set_ipfs '{"ipfs": "ipfs://bafybeihiw6mwe63nvy6it7sialrr3fgx57jqnfdqlpxg7do6fytbxnzmna"}' --accountId YOUR_ACCOUNT.near
  ```
8. Store Web4 Profile WASM bytes inside Factory Contract
  ```sh
  CONTRACT_BYTES=`cat ./res/contract.wasm | base64`
  ```
  ```sh
  NODE_ENV=mainnet near call page_factory.YOUR_ACCOUNT.near set_code "$CONTRACT_BYTES" --base64 --accountId YOUR_ACCOUNT.near
  ```

## Usage

You can easily create your site with a single function call

To gain ownership of your created Web4 account, you need to replace `YOUR_PUBLIC_KEY` with your own public key from the FullAccess key pair

For instance, my public key is `ed25519:8W9CiyPPehz2GRW8AYho9nx1z1GLdeZQCyn2wqYgJjiG`

```sh
NODE_ENV=mainnet near call page_factory.YOUR_ACCOUNT.near create_web4_little_link_page '{"page_data":{"title":"Near Ukraine Guild","description":"Fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem", "links":[{
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
    }]}, "pub_key": "YOUR_PUBLIC_KEY"}' --accountId denbite.testnet --deposit 0.7 --gas 300000000000000
```

The function will provide the address of the deployed contract, which will look like `d5f6574f11d34497671dee4e.near`

You can then access the site at https://d5f6574f11d34497671dee4e.near.page by replacing the address with the account that was created for you

---

## Developed by

![Guild cover][cover]

**Near Ukraine Guild** is a fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem

## Community Validator Node

![Community Validator cover][validator]

Our validator has been active for a few months now, and the funds it generates are being put towards sponsoring community activities

Join us now to stake and earn 10% APY

**Click below to get started 👇**

<a href="https://bit.ly/43GSKhs" target="_blank">
<img src="https://img.shields.io/badge/stake-red?style=for-the-badge"  height="48" />
</a>

## Socials

[![Twitter][twitter]][twitter-url]
[![Youtube][youtube]][youtube-url]
[![Telegram Chat][telegram-chat]][telegram-chat-url]
[![Telegram Channel][telegram-channel]][telegram-channel-url]
[![Medium][medium]][medium-url]
[![Github][github]][github-url]

<!-- Images -->

[cover]: https://github.com/nearuaguild/.github/blob/main/images/cover.png
[validator]: https://github.com/nearuaguild/.github/blob/main/images/validator.png

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

<!-- CTA -->

[stake]: https://img.shields.io/badge/stake-yellow?style=for-the-badge
[stake-url]: https://bit.ly/43GSKhs

<!-- LICENSE -->

## License

See `LICENSE.txt` for more information

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

<!-- Built with -->

[rust]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/

[javascript]: https://img.shields.io/badge/javascript-000000?style=for-the-badge&logo=javascript&logoColor=F7E018
[javascript-url]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
[assemblyscript]: https://img.shields.io/badge/assembly%20script-1B7ACE?style=for-the-badge&logo=assemblyscript&logoColor=white
[assemblyscript-url]: https://www.assemblyscript.org/
[littlelink]: https://img.shields.io/badge/LittleLink-1D84FF?style=for-the-badge
[littlelink-url]: https://littlelink.io/