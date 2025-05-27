package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"math/rand/v2"
	"time"

	viewer "github.com/ALifeComponent/human-interface-engine/gen/go/viewer/v1"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// ■ Custom Duration Flag
//
//	-wait        → use the default value (500ms)
//	 -wait=200ms → use 200ms
type DurationFlag struct {
	Duration time.Duration
	Default  time.Duration
}

func (d *DurationFlag) String() string { return d.Duration.String() }
func (d *DurationFlag) Set(s string) error {
	// "-wait" だけ、もしくは "-wait=true" でデフォルトを適用
	if s == "" || s == "true" {
		d.Duration = d.Default
		return nil
	}
	v, err := time.ParseDuration(s)
	if err != nil {
		return err
	}
	d.Duration = v
	return nil
}
func (d *DurationFlag) IsBoolFlag() bool { return true }

// Delay flag for SpawnObjectSequence RPC
var spawnWait = &DurationFlag{Default: 500 * time.Millisecond, Duration: 500 * time.Millisecond}

// Delay flag for SetObjectPositionSequence RPC
var setPositionWait = &DurationFlag{Default: 500 * time.Millisecond, Duration: 500 * time.Millisecond}

var target string = "localhost:50051"

func init() {
	flag.Var(spawnWait, "spawn-wait", fmt.Sprintf(
		"delay before SpawnObjectSequence RPC (default = %v or -spawn-wait=<duration>)",
		spawnWait.Default,
	))
	flag.Var(setPositionWait, "set-position-wait", fmt.Sprintf(
		"delay before SetObjectPositionSequence RPC (default = %v or -set-position-wait=<duration>)",
		setPositionWait.Default,
	))
	// Network Host & port
	flag.StringVar(&target, "target", target, fmt.Sprintf(
		"target server address (default = %s or -target=<host:port>)",
		target,
	))
}

func main() {
	flag.Parse()
	conn, err := grpc.NewClient(target, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	defer conn.Close()

	client := viewer.NewManageObjectServiceClient(conn)

	ctx := context.Background()

	// 100 sequence messages filled with random data
	var reqs *viewer.SpawnObjectSequenceRequest = &viewer.SpawnObjectSequenceRequest{
		Requests: make([]*viewer.SpawnObjectRequest, 100),
	}
	for i := range 100 {
		reqs.Requests[i] = makeRandomSpawnObjectRequest()
	}

	var uuids [][]byte = make([][]byte, 100*100)

	// Send 100 requests of SpawnObjectSequenceRequest thath contains 100 SpawnObjectRequest (Spawing 10000 objects)
	for i := range 100 {
		time.Sleep(spawnWait.Duration)
		resp, err := client.SpawnObjectSequence(ctx, reqs)
		if err != nil {
			log.Fatalf("RPC failed: %v", err)
		}

		for j, r := range resp.Responses {
			log.Printf("Response: %v", r)
			if r.SpawendObjectId.Uuid != nil {
				uuids[j+i] = r.SpawendObjectId.Uuid.Value
			}
		}
	}

	for range 1000 {

		// Send 100*100 requests of `SetObjectPositionRequest` that contains 100 `SetObjectPositionRequest` (Moving 10000 objects)
		for i := range 100 {
			time.Sleep(setPositionWait.Duration)
			var reqs2 *viewer.SetObjectPositionSequenceRequest = &viewer.SetObjectPositionSequenceRequest{
				Requests: make([]*viewer.SetObjectPositionRequest, 100),
			}
			for j := range 100 {
				reqs2.Requests[j] = makeRandomSetObjectPositionRequest(uuids[j+i])
			}

			resp2, err := client.SetObjectPositionSequence(ctx, reqs2)
			if err != nil {
				log.Fatalf("RPC failed: %v", err)
			}

			for _, r := range resp2.Responses {
				log.Printf("Response: %v", r)
			}

		}
	}
}

func makeRandomSetObjectPositionRequest(uuid []byte) *viewer.SetObjectPositionRequest {
	randomPosition := &viewer.Vector3{
		X: rand.Float32() * 100,
		Y: rand.Float32() * 100,
		Z: rand.Float32() * 100,
	}

	return &viewer.SetObjectPositionRequest{
		ObjectId: &viewer.ObjectId{
			Uuid: &viewer.Uuid{
				Value: uuid,
			},
		},
		Position: randomPosition,
	}
}

func makeRandomSpawnObjectRequest() *viewer.SpawnObjectRequest {
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
