# Semantic Search on stack exchange data

Stack Exchange kindly provides the data from all its communities under a permissive Creative Common License. The data dumps can be downloaded from here: <https://data.stackexchange.com/>.

This **Proof of Concepts** explore how we could improve searching for stack overflow issues (or even generate new answers based on existing ones) using Large Language Models.

The most famous of these communities is certainly <https://stackoverflow.com/>. **Attention** do not use these tool on stackoverflow, yet. It does not scale **that** well (yet). If you want to download some data from the smaller communities like *3D Printing* or *Health*, go for it. See how it actually works really well (so far, from what I can tell).

## Usage

```bash
search-stack-exchange question health-Posts.xml "Is showering bad for my skin?"
```

Standard out will show the best match of title which fits your question:

```
Is there any health benefit or detriment from bathing?
```

## Installation

1. Okay, first you need the executable. Currently it is not deployed anythere so you need to checkout this repository and build it from source using a rust toolchain. You can install rust from here: <http://rustup.rs>
2. Checkout this repository using git: `git clone https://github.com/pacman82/search-stack-exchange.git`
3. Change directory into your local repostory and bulid and install the tool: `cargo install --path .`
4. Now we need some data to run it on. You can download it yourself using a browser from <https://data.stackexchange.com/> or invoke the script provided with this repository. Let's say you want to find health related issues: `sh download_stackexchange health`
5. Finally we need an aleph alpha API Token. You can sign up for an account here: <https://app.aleph-alpha.com>. Go to profile to obtain an API token. The service is not for free, and the free tokens you get for sign up, won't be enough to embed the data for the entire health community, yet 5 euro is enough. Minimum transaction on the side is 10 euro though.
6. Now with an api token, data, tool and credits you can finally ask your first question.

The first answer will take a while, since all the titles you downloaded needs to be processed. After that it should be (almost) instant.
  

## License

The data used in the integeration tests of this crate is copy and pasted from stack exchange data. See: <https://data.stackexchange.com/>