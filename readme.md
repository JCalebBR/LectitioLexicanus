<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/JCalebBR/LectitioLexicanus">
    <img src="images/logo.png" alt="Logo" width="50%" height="50%">
  </a>

  <p align="center">
    A Lexicanum-based dictionary for Kindles.
    <br />
    ·
    <a href="https://github.com/JCalebBR/LectitioLexicanus/issues">Report Bug</a>
    ·
    <a href="https://github.com/JCalebBR/LectitioLexicanus/issues">Request Feature</a>
  </p>

[![Pre-release Build](https://github.com/JCalebBR/LectitioLexicanus/actions/workflows/prerelease.yml/badge.svg)](https://github.com/JCalebBR/LectitioLexicanus/actions/workflows/prerelease.yml)
[![Stable Build](https://github.com/JCalebBR/LectitioLexicanus/actions/workflows/release.yml/badge.svg)](https://github.com/JCalebBR/LectitioLexicanus/actions/workflows/release.yml)
<br />
[![](https://dcbadge.vercel.app/api/server/pjQWRWknd4)](https://discord.gg/pjQWRWknd4)
</div>


## Table of Contents
<!-- TABLE OF CONTENTS -->
- [Table of Contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Downloading / Installing](#downloading--installing)
- [Local build](#local-build)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)


<!-- ABOUT THE PROJECT -->
## About The Project

<p float="left">
    <img src="images/example%20(1).png" alt= “” width="35%" height="35%">
    <img src="images/example%20(2).png" alt= “” width="35%" height="35%">
    <img src="images/example%20(3).png" alt= “” width="35%" height="35%">
    <img src="images/example%20(4).png" alt= “” width="35%" height="35%">
</p>
<br />
This project is a dictionary for the Warhammer 40k universe, based on the Lexicanum website. It is intended to be used on Kindles, but can be used on any device that supports MOBI files.

The dictionary is automatically generated after parsing and filtering a dump of allpages of the Lexicanum website. The dictionary is then converted into a MOBI file using Amazon's `kindlegen.exe` tool.

It offers support for inflections, based on redirects of a page. For example, the page "Neverborn" redirects to "Daemon". By highlighting either word, one would be able to see the same definition.

You can also join the [discord](https://discord.gg/pjQWRWknd4) to keep up with updates and changes

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Downloading / Installing

In order to obtain the latest version of the dictionary, you can download the latest release [here][releases-url]. 

Connect your kindle via USB, open the `documents` folder and paste the MOBI file into the `dictionaries` folder. It should be available for use after that.

If you already have an older version and wish to update, simply delete the old MOBI file and paste the new one in its place. Then restart your Kindle.

Note that, the pre-releases are always the latest version of the dictionary, but may not be stable and can also have bigger filesizes due to no compression.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Local build

### Prerequisites

* cargo
  ```sh
  cargo install
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/JCalebBR/LectitioLexicanus.git
   ```
2. Run `cargo run --release` to obtain the latest version of the Lexicanum Dump. This will create a file called `filtered-output.jsonl` which will be used to create the dictionary. This will create a file called `content.html` in the `dict` directory, which will be the contents of the dictionary.
   
   **Note that this will take approximately 5 minutes**
    ```sh	
    cargo run --release
    ```

3. Run `kindlegen.exe` to create the MOBI file. This will create a file called `dict.mobi` in the `dict` directory, which will be the dictionary.
    ```sh	
    kindlegen.exe dict/dict.opf -c2 -verbose -dont_append_source
    ```


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [ ] Reduce number of entries by associating entries with a parent entry. For example, "Theron (Planet)" and "Theron (Squad)" could be associated with "Theron (Ultramarines)".
- [ ] Add better formatting to the dictionary entries.

See the [open issues](https://github.com/JCalebBR/LectitioLexicanus/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the GNU GPLv3. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Lexicanum](https://wh40k.lexicanum.com/) - For hosting their wiki and making such a project possible
* [Jake McCrary](https://jakemccrary.com) - For his insightful documentation on creating a Kindle dictionary.
* [Games Workshop and The Black Library](https://www.blacklibrary.com/) - For creating the Warhammer 40k universe.
  
<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[example-01]: images/example-01.png
[example-02]: images/example-02.png
[example-03]: images/example-03.png
[example-04]: images/example-04.png
[releases-url]: https://github.com/JCalebBR/LectitioLexicanus/releases
[Python]: https://img.shields.io/badge/python-000000?style=for-the-badge&logo=python
[Python-url]: https://www.python.org/
