# **AutoGrader** : Programming Competition Scoreboard

**AutoGrader** is a `Rust` Programming competition scoreboard which can use on both CLI and TCP/IP protocol.

*This repo contains only ***core*** of AutoGrader not included the frontend which can easily implement API to send the submissions files*


### *You need [Rustup](https://rustup.rs/) to compile AutoGrader!!!*

<br/>

## **Installation**
*TBD*

## **Configuration**
* ### **Test Config**:
  Use to configure how every test being
```toml
id = "1" # Optional
name = "Sum two numbers"
info = "Calculate some from two numbers provided"
decription = { type: "HTML", path: "/path/to/desc.html" }
num_testcase = 10
score = 100
files = ['testcase1.txt','testcase2.txt',....]
```
