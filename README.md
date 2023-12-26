<p align="center">
  <a href="https://github.com/Julgodis/kotoba/">
    <h1 align="center">
      Kodama
    </h1>
  </a>
</p>

<div align="center">

[![license](https://img.shields.io/crates/l/picori)](https://github.com/Julgodis/genkei/LICENSE)

```diff
!!! This project is primarily for personal use, and I may not actively accept !!!
!!! pull requests. Feel free to explore the code, but please understand that  !!!
!!! contributions might not be incorporated.                                  !!!
```

</div>

Kodama is a simple data collecting server and API. It currently supports the following data types:
- [X] Metrics (e.g. CPU usage, RAM usage, etc.)
- [X] Records (e.g. how long rendering /index.html took)
- [ ] Logs (e.g. log messages from a service)
- [ ] Events (e.g. a user logged in)
- [ ] Alerts (e.g. a record p99 is above 100ms)

The data is organized in project with services. Each service can have multiple metrics and records. Kodama also includes a query builder for SQL databases with built-in support to record the querys as records.

##  License
This project is licensed under the MIT License. See the LICENSE file for details.