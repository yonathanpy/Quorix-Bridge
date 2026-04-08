package main

import (
	"quorix-bridge/internal/api"
	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()
	api.RegisterRoutes(r)
	r.Run(":8080")
}
