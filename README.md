# Spring4shell PoC Lab
Spring 4 Shell PoC script writted in Rust

## Vulnerable Products
- JDK version 9.0+
- Spring framework and derivative framework spring-beans-*.jar exists

## Checking Affected System

- For JDK version, you can use 

```bash
java -version
```

- To check if you are using Spring framework or derivative. Do a global search after "spring-beans-.jar" and "spring.jar"

```bash
find . -name spring-beans*.jar
```


## Detail Information
- https://www.cyberkendra.com/2022/03/springshell-rce-0-day-vulnerability.html
- https://www.springcloud.io/post/2022-03/spring-0day-vulnerability/#gsc.tab=0
- https://www.praetorian.com/blog/spring-core-jdk9-rce/

## Spring Patch
- https://github.com/spring-projects/spring-framework/commit/7f7fb58dd0dae86d22268a4b59ac7c72a6c22529

## Acknowledgment

- https://github.com/Retrospected/spring-rce-poc
- https://github.com/craig/SpringCore0day
- https://github.com/BobTheShoplifter/Spring4Shell-POC