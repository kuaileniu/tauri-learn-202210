package main

import (
	"github.com/kuaileniu/zlog"
	"go.uber.org/zap"
)

func main(){
	// /Volumes/HD/tmp
	zlog.InitLogger(zlog.LogConfig{Filename: "/Volumes/HD/tmp/tauri-call-go.log", Console: true})
	zap.L().Info("tauri call elf(from golang) success");
}