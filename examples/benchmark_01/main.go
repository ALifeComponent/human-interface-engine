package main

import (
	"context"
	"log"
	"math/rand/v2"
	"time"

	viewer "github.com/ALifeComponent/human-interface-engine/gen/go/viewer/v1"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	defer conn.Close()

	client := viewer.NewManageObjectServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	makeRandomRequest := func() *viewer.SpawnObjectRequest {
		randomPosition := &viewer.Vector3{
			X: rand.Float32() * 100,
			Y: rand.Float32() * 100,
			Z: rand.Float32() * 100,
		}

		randomSize := rand.Float32() * 10

		randomColor := &viewer.RGBA{
			R: rand.Float32(),
			G: rand.Float32(),
			B: rand.Float32(),
			A: 1.0,
		}

		randomShape := rand.UintN(2) + 1

		return &viewer.SpawnObjectRequest{
			ObjectProperties: &viewer.ObjectProperties{
				Shape: viewer.ObjectShape(randomShape),
				Size: &viewer.ObjectSize{
					Value: randomSize,
				},
				Color: &viewer.ObjectColor{
					Color: &viewer.ObjectColor_ColorRgba{
						ColorRgba: randomColor,
					},
				},
			},
			Position: randomPosition,
		}
	}

	// 100 sequence messages filled with random data
	var reqs *viewer.SpawnObjectSequenceRequest = &viewer.SpawnObjectSequenceRequest{
		Requests: make([]*viewer.SpawnObjectRequest, 100),
	}
	for i := range 100 {
		reqs.Requests[i] = makeRandomRequest()
	}

	// Send 100 requests of SpawnObjectSequenceRequest thath contains 100 SpawnObjectRequest (Spawing 10000 objects)
	for range 100 {
		resp, err := client.SpawnObjectSequence(ctx, reqs)
		if err != nil {
			log.Fatalf("Echo RPC failed: %v", err)
		}

		for _, r := range resp.Responses {
			log.Printf("Response: %v", r)
		}
	}
}
