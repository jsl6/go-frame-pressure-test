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
