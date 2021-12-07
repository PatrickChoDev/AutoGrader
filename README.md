# **AutoGrader** : Programming Competition Scoreboard

**AutoGrader** is a `Rust` Programming competition scoreboard which can use on both CLI and TCP/IP protocol.

\*This repo contains only **_core_** of AutoGrader not included the frontend which can easily implement API to send the submissions files\*

### _You need [Rustup](https://rustup.rs/) to compile AutoGrader!!!_

<br/>

## **Installation**

---

- Any Linux Distribution
- [Docker](https://docker.com) installed with user added to group, `docker`.
- If API can't reach please confirm you added the firewall rules to the `port` in [Session Config](#SessionConfig)
- *TBD next...*

## **Configuration**

---

- ### <a name="#SessionConfig"></a>**Session Config**:
  - Use to configure session for `serve` subcommand.
  - If `port` is less than `1000` **AutoGrader** required to use `sudo` privilege to listening on incoming request via **API**. *(Not recommend)*
  - `scheme` is use to define database table on database named in `db` field.
  - `group_score` is an optional for individual scoring or when `{"group":false}`.

```json
{
  "name": "AutoGrader Test",
  "port": 5000,
  "database": {
    "host": "127.0.0.1",
    "port": 12345,
    "username": "autograder",
    "password": "a4t0gr4d3r",
    "mode": "pgsql",
    "scheme": {
      "db": "autograder_session",
      "user": "user",
      "score": "score",
      "group_score": "group_score"
    }
  },
  "runner": {
    "host": "127.0.0.1",
    "port": 15000
  },
  "test_dir": "~/",
  "group": true
}
```

- ### **Test Config**:
  - Use to configure test and cases for `run` and `serve` subcommand.
  - If `pass_all` represented in config file, `score_weight` will be ignored and will use `score` instead.
  - If `score_weigth` is unequal to `score`, **AutoGrader** will normalize sum of `score_weight` to `score` automatically, or else sum of `score_weight` is `score` if `score` isn't represented.

```json
{
  "name": "Sum the number",
  "info": "Sum the numbers given in array",
  "description": {
    "reader": "md",
    "content": "sum/sum.desc.md"
  },
  "score": 100,
  "cases": {
    "dir": "sum",
    "solution": "sum/sol.cpp",
    "pass_all": true,
    "score_weight": {
      "sum1.txt": 60,
      "sum2.txt": 40
    }
  },
  "limit": {
    "cpu": 1000,
    "memory": "256M",
    "time": 1000
  }
}
```
