# 对GO 主流框架 和 Rust的actix web进行Pressure Test对比

开始我们的Pressure Test,欢迎star

## 环境：
MacBook Pro (15-inch, 2017)  
CPU: 3.1 GHz Intel Core i7  
Mem: 16 GB 2133 MHz LPDDR3  

环境设置  
ulimit -n 1048576

### 1. Go gin Framework
```golang
package main

import (
	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.New()
	r.GET("/", func(ctx *gin.Context) {
		ctx.Writer.Write([]byte("Hello World"))
	})
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}

```
Run
```shell
$ go run hello-world/main_gin.go 
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
Date: Sat, 08 Feb 2020 07:48:12 GMT
Content-Length: 11
Content-Type: text/plain; charset=utf-8

Hello World%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.33ms    3.39ms  90.62ms   83.61%
    Req/Sec     6.97k   773.90    12.75k    77.71%
  5547468 requests in 1.67m, 677.18MB read
  Socket errors: connect 0, read 848, write 8, timeout 0
Requests/sec:  55417.42
Transfer/sec:      6.76MB
```
CPU: 260% - 330% 内存: 30m



### 2. Go iris Framework
code 
```golang
package main

import "github.com/kataras/iris/v12"

func main() {
	app := iris.New()
	app.Get("/", func(ctx iris.Context) {
		ctx.Write([]byte("Hello World"))
	})

	app.Run(iris.Addr(":8080"))
}

```
Run
```shell
$ go run hello-world/main_iris.go 
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
Date: Sat, 08 Feb 2020 07:51:49 GMT
Content-Length: 11
Content-Type: text/plain; charset=utf-8

Hello World%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.24ms    3.53ms  68.91ms   83.17%
    Req/Sec     6.98k   785.89    11.66k    78.38%
  5558509 requests in 1.67m, 678.53MB read
  Socket errors: connect 0, read 925, write 0, timeout 0
Requests/sec:  55542.67
Transfer/sec:      6.78MB
```
CPU: 260% - 332% 内存：33m

### 3. Go echo Framework
code
```golang
// https://echo.labstack.com
package main

import (
	"net/http"

	"github.com/labstack/echo"
)

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello World")
	})
	e.Start(":8080")
}

```
Run
```shell
$ go run hello-world/main_echo.go 
```
Test
```curl -i http://localhost:8080
HTTP/1.1 200 OK
Content-Type: text/plain; charset=UTF-8
Date: Sat, 08 Feb 2020 07:55:32 GMT
Content-Length: 11

Hello World%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.50ms    3.72ms 113.97ms   85.23%
    Req/Sec     6.87k   835.97    11.97k    75.54%
  5466944 requests in 1.67m, 667.35MB read
  Socket errors: connect 0, read 915, write 0, timeout 0
Requests/sec:  54616.08
Transfer/sec:      6.67MB
```
CPU: 290% - 340% 内存： 29m

### 3. Go chi Framework
code
```golang
/// go chi framework
package main

import (
  "net/http"
  
	"github.com/go-chi/chi"
)

func main() {
	r := chi.NewRouter()
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Hello World"))
	})
	http.ListenAndServe(":8080", r)
}

```
Run
```shell
$ go run hello-world/main_chi.go 
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
Date: Sat, 08 Feb 2020 07:58:43 GMT
Content-Length: 11
Content-Type: text/plain; charset=utf-8

Hello World%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.13ms    3.34ms  78.01ms   84.38%
    Req/Sec     7.02k   725.59    11.73k    80.36%
  5590349 requests in 1.67m, 682.42MB read
  Socket errors: connect 0, read 933, write 0, timeout 0
Requests/sec:  55867.91
Transfer/sec:      6.82MB
```
CPU:  内存: 28m

### 4. Go restful Framework
code
```golang
package main

import (
	"net/http"

	"github.com/emicklei/go-restful"
)

func main() {
	ws := new(restful.WebService)
	ws.Route(ws.GET("/").To(hello))
	restful.Add(ws)
	http.ListenAndServe(":8080", nil)
}
func hello(req *restful.Request, resp *restful.Response) {
	resp.Write([]byte("Hello World"))
}

```
Run
```shell
$ go run hello-world/main_restful.go 
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
Date: Sat, 08 Feb 2020 08:01:33 GMT
Content-Length: 11
Content-Type: text/plain; charset=utf-8

Hello World%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.17ms    3.83ms  80.43ms   83.28%
    Req/Sec     6.95k   828.46    12.10k    81.01%
  5537873 requests in 1.67m, 676.01MB read
  Socket errors: connect 0, read 1043, write 0, timeout 0
Requests/sec:  55325.72
Transfer/sec:      6.75MB
```
CPU: 290% - 360% 内存：33m


### 5. Rust actix web
最近Rust很火，性能也很好，也那里对比一下:
### Rust actix_web 1.0 Framework
```shell
$ cargo new rust-actix-web-v1
$ cd rust-actix-web-v1
```

src/main.rs:
```code
use actix_web::{web, App, HttpServer};

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| "Hello world"))
    })
    .bind("0.0.0.0:8080")?
    .run()
}

```
Run
```shell
$ cd hello-world/rust-actix-web-v1
$ cargo run --release
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
content-length: 11
content-type: text/plain; charset=utf-8
date: Sat, 08 Feb 2020 07:45:41 GMT

Hello world%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.46ms    2.66ms  51.33ms   85.82%
    Req/Sec     8.89k     1.13k   13.72k    77.51%
  7078597 requests in 1.67m, 864.09MB read
  Socket errors: connect 0, read 1054, write 0, timeout 0
Requests/sec:  70744.27
Transfer/sec:      8.64MB
```
CPU: 200% - 230% 内存：72m


### 6. Rust actix_web 2.0 Framework
```shell
$ cargo new rust-actix-web-v2
$ cd rust-actix-web-v2
```
src/main.rs:
```code
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```
Run
```shell
$ cd hello-world/rust-actix-web-v2
$ cargo run --release
```
Test
```shell
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
content-length: 11
content-type: text/plain; charset=utf-8
date: Sat, 08 Feb 2020 08:37:31 GMT

Hello world%
```
Pressure Test
```shell
$ wrk -d 100s -c 1000 -t 8 http://localhost:8080
Running 2m test @ http://localhost:8080
  8 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.92ms    3.26ms 102.39ms   81.81%
    Req/Sec     8.66k     1.03k   12.01k    73.72%
  6893954 requests in 1.67m, 841.55MB read
  Socket errors: connect 0, read 1118, write 0, timeout 0
Requests/sec:  68870.88
Transfer/sec:      8.41MB
```
CPU: 170% - 230% 内存：53m

## 7. 以上显示
Name | CPU | Mem | QPS |
|---|:---:|---|---|
| [Go Gin](https://github.com/gin-gonic/gin)  | 260% - 330% | 30m | 55417 |
| [Go Iris](https://github.com/kataras/iris) | 260% - 332% | 33m | 55542 |
| [Go Echo](https://github.com/labstack/echo) | 260% - 330% | 30m | 54616 |
| [Go Revel](https://github.com/revel) | |  |  |
| [Go Buffalo](https://github.com/gobuffalo/buffalo) |  |  |  |
| [Go Chi](https://github.com/go-chi/chi) | 290% - 340% | 28m | 55867 |
| [Go Restful](https://github.com/emicklei/go-restful) | 290% - 360% | 33m | 55325 |
| [Rust actix-web 1.0](https://github.com/actix/actix-web) | 200% - 230% | 72m | 70744 |
| [Rust actix-web 2.0](https://github.com/actix/actix-web) | 170% - 230% | 53m | 68870 |

以上数据仅供参考


参考资料：  
1. Web Framework Benchmarks   
https://www.techempower.com/benchmarks/#section=data-r18  
2. Awesome Web Frameworks for Gophers  
https://raw.githubusercontent.com/speedwheel/awesome-go-web-frameworks/master/README.md


3. https://github.com/speedwheel/awesome-go-web-frameworks/blob/master/README.md#popularity:

[Go](https://golang.org) is a rapidly growing open source programming language designed for building simple, fast, and reliable software. Take a look [here](https://github.com/golang/go/wiki/GoUsers) to see which great companies use Go to power their services.

This repository has all the necessary information to help developers learn more about the best options that are out there to develop web applications with Go.

The repository contains the most detailed framework comparison that is out there, by comparing the the most known web frameworks from as many angles as possible: popularity, support and built'n features:

**Beego**: _An open-source, high-performance web framework for the Go programming language._
* https://github.com/astaxie/beego
* https://beego.me

**Buffalo**: _Rapid Web Development w/ Go._
* https://github.com/gobuffalo/buffalo
* https://gobuffalo.io

**Echo**: _A high performance, minimalist Go web framework._
* https://github.com/labstack/echo
* https://echo.labstack.com

**Gin**: _HTTP web framework written in Go (Golang). It features a Martini-like API with much better performance._
* https://github.com/gin-gonic/gin
* https://gin-gonic.github.io/gin

**Iris**: _The fastest web framework for Go in The Universe. MVC fully featured. Embrace the future today_
* https://github.com/kataras/iris
* https://iris-go.com

**Revel**: _A high productivity, full-stack web framework for the Go language._
* https://github.com/revel/revel
* https://revel.github.io

4. Popularity(2020-02-10)

> Sorted by the popularity (stars)

| Name | Stars | Forks | Issues Open | Issues Closed  | Birth Year | Latest Update | Author |
|---|---|---|---|---|----|----|----|
| [Gin](https://github.com/gin-gonic/gin) | 35,235 | 4.1k | 192 |  1125 |  2014 | 2020-02-08 | [@manucorporat](https://github.com/manucorporat) |
| [Beego](https://github.com/astaxie/beego) | 23,188 | 4.7k | 754 | 1902 | 2012  | 2020-02-08 | [@astaxie](https://github.com/astaxie) |
| [Iris](https://github.com/kataras/iris) | 17,428 | 1.9k | 8 |  485 | 2016 | 2020-02-07 | [@kataras](https://github.com/kataras) |
| [Echo](https://github.com/labstack/echo) | 16,433 | 1.5k | 32 |  920 |  2015 | 2020-02-04 | [@vishr](https://github.com/vishr) |
| [Revel](https://github.com/revel) | 11,562 | 1.4k | 81 |  832 | 2011 | 2018-10-30 | [@revel](https://github.com/revel) |
| [Buffalo](https://github.com/gobuffalo/buffalo) | 5,347 | 426 | 63 |  714 |  2014 | 2020-02-07 | [@markbates](https://github.com/markbates) |

5. Learning Curve

Name | Examples | Reference | Real time support |
|---|:---:|---|---|
| Iris | 92 | https://github.com/kataras/iris/tree/master/_examples | [rocket chat](https://chat.iris-go.com) |
| Beego | 49 | https://beego.me/docs | |
| Echo | 20 | https://echo.labstack.com/cookbook/hello-world | |
| Gin | 18 | https://github.com/gin-gonic/gin/tree/master/examples | [gitter](https://gitter.im/gin-gonic/gin) |
| Revel | 6 | http://revel.github.io/examples/index.html | [gitter](https://gitter.im/revel/community) |
| Buffalo | 6 | https://gobuffalo.io/docs/installation | [slack](https://gophers.slack.com/messages/buffalo/) |

Great job by astaxie and kataras here, hopfully and the other frameworks will catch up with more examples, at least for me, if I switch to a new framework, that's the most resourceful place to quickly grasp as much information as possible. An example it's like 1000 words.

6. Core Features

> Sorted by the most to less featured

| Name |Iris|Beego|Revel|Echo|Gin|Buffalo|
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| Router: Named Path Parameters & Wildcard[*](#router-named-path-parameters--wildcard) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- |
| Router: Regex[*](#router-regex) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png">  | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- |
| Router: Grouping[*](#router-grouping) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | ❔ | ❔ | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- |
| Router: All the above Mixed Without Conflict[*](#all-the-above-mixed-without-conflict) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Router: Custom HTTP Errors[*](#router-custom-http-errors) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |
| 100% compatible with net/http[*](#100-compatible-with-nethttp) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png">  |  ❔ | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">  | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |
| Middleware ecosystem[*](#middleware-ecosystem) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Sinatra-like API[*](#sinatra-like-api) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Server: Automatic HTTPS[*](#server-automatic-https) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | ❔| <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Server: Gracefully Shutdown[*](#server-gracefully-shutdown) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Server: Multi Listeners[*](#server-multi-listeners) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Full HTTP/2[*](#full-http2) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | ❔| ❔ | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | standard | standard |
| Subdomains[*](#subdomains) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | secondary | secondary |  secondary |  secondary | secondary |
| Sessions[*](#sessions) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | secondary |
| Websockets[*](#websockets) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View (aka Templates) Embedded Into App[*](#view-aka-templates-embedded-into-app) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View Engine: STD[*](#view-engine-std) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View Engine: Pug[*](#view-engine-pug) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View Engine: Django[*](#view-engine-django) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View Engine: Handlebars[*](#view-engine-handlebars) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| View Engine: Amber[*](#view-engine-amber) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Renderer: Markdown, JSON, JSONP, XML...[*](#renderer-markdown-json-jsonp-xml) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| MVC[*](#mvc) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- | generator | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Caching[*](#catching) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">| <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| File Server[*](#file-server) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> |
| File Server: Embedded Into App[*](#file-server-embedded-into-app) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Response can be Modified Many times through lifecycle before sent[*](#response-can-be-modified-many-times-through-lifecycle-before-sent) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |  <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Gzip[*](#gzip) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | middleware | middleware | middleware |
| Testing Framework[*](#testing-framework) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Typescript Transpiler[*](#typescript-transpiler) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Online Editor[*](#online-editor) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Logging System[*](#logging-system) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png">- | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Maintenance & Auto-Updates[*](#maintenance--auto-updates) | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2714.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> | <img height="15" width="15" src="https://github.githubassets.com/images/icons/emoji/unicode/2796.png"> |
| Performance | ★★★★★ | ★★★ | ★★ | ★★★★★ | ★★★★★ | ★★★ |

```