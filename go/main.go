package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"net"
	"sync/atomic"
	"time"

	"google.golang.org/grpc"
	pb "rustvsgo/helloworldpb"
)

var requestCount uint64

type greeterServer struct {
	pb.UnimplementedGreeterServer
}

func (s *greeterServer) SayHello(ctx context.Context, req *pb.HelloRequest) (*pb.HelloReply, error) {
	atomic.AddUint64(&requestCount, 1)

	reply := &pb.HelloReply{
		Message:      fmt.Sprintf("Hello %s, age %d", req.Name, req.Age),
		UserId:       rand.Int63(),
		Score:        float64(req.Rating) + 10.0,
		Active:       req.Subscribed,
		LuckyNumbers: []int32{4, 8, 15},
		Metadata:     map[string]string{"source": "go-grpc"},
	}

	return reply, nil
}

func startStatsPrinter() {
	go func() {
		for {
			time.Sleep(10 * time.Second)
			count := atomic.SwapUint64(&requestCount, 0)
			log.Printf("[Go Server] Total requests in the past 10 seconds: %d", count)
		}
	}()
}

func main() {
	addr := "0.0.0.0:5000"
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	startStatsPrinter()

	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &greeterServer{})

	log.Printf("Go server listening on %s", addr)
	if err := s.Serve(listener); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
