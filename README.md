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

Kodama is a data collection server and API designed for simplicity and effectiveness. It supports various data types, organized into the following categories:

- [X] Metrics: Includes data such as CPU usage and RAM usage.
- [X] Records: Covers time-specific data, like the time taken to render /index.html.
- [ ] Logs: Will capture log messages from services.
- [ ] Events: To track occurrences such as user logins.
- [ ] Alerts: For monitoring critical metrics, e.g., a record's p99 exceeding 100ms.

Data within Kodama is structured by projects, with each project containing multiple services. Each service is capable of handling multiple metrics and records. Additionally, Kodama features a query builder for SQL databases, which includes functionality to record these queries as part of its data collection.

##  License
This project is licensed under the MIT License. See the LICENSE file for details.
