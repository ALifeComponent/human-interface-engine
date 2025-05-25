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
//	-wait=200ms → use 200ms
type DurationFlag struct {
	Duration time.Duration
	Default  time.Duration
}

func (d *DurationFlag) String() string { return d.Duration.String() }
func (d *DurationFlag) Set(s string) error {
	// Use default value when "-wait" or "-wait=true" is provided
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

func init() {
	flag.Var(spawnWait, "spawn-wait", fmt.Sprintf(
		"delay before SpawnObjectSequence RPC (default = %v or -spawn-wait=<duration>)",
		spawnWait.Default,
	))
	flag.Var(setPositionWait, "set-position-wait", fmt.Sprintf(
		"delay before SetObjectPositionSequence RPC (default = %v or -set-position-wait=<duration>)",
		setPositionWait.Default,
	))
}

func main() {
	flag.Parse()
	client, ctx := setupClient()
	defer client.conn.Close()

	ids := runSpawnSequences(ctx, client.stub)
	runPositionSequences(ctx, client.stub, ids)
}

// setupClient establishes a gRPC connection and returns the client and context
func setupClient() (struct {
	conn *grpc.ClientConn
	stub viewer.ManageObjectServiceClient
}, context.Context) {
	conn, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	client := viewer.NewManageObjectServiceClient(conn)
	return struct {
		conn *grpc.ClientConn
		stub viewer.ManageObjectServiceClient
	}{conn, client}, context.Background()
}

// runSpawnSequences executes the object spawn sequence and returns the generated UUIDs
func runSpawnSequences(ctx context.Context, client viewer.ManageObjectServiceClient) [][]byte {
	// Assemble requests
	reqs := &viewer.SpawnObjectSequenceRequest{Requests: make([]*viewer.SpawnObjectRequest, 100)}
	for i := range reqs.Requests {
		reqs.Requests[i] = makeRandomSpawnObjectRequest()
	}

	ids := make([][]byte, len(reqs.Requests)*len(reqs.Requests))
	for i := range reqs.Requests {
		time.Sleep(spawnWait.Duration)
		resp, err := client.SpawnObjectSequence(ctx, reqs)
		if err != nil {
			log.Fatalf("Spawn RPC failed: %v", err)
		}
		for j, r := range resp.Responses {
			log.Printf("Spawn Response: %v", r)
			if r.SpawendObjectId.Uuid != nil {
				ids[j+i] = r.SpawendObjectId.Uuid.Value
			}
		}
	}
	return ids
}

// runPositionSequences executes the object position sequence
func runPositionSequences(ctx context.Context, client viewer.ManageObjectServiceClient, ids [][]byte) {
	for range 1000 {
		for i := range ids {
			time.Sleep(setPositionWait.Duration)
			reqs := &viewer.SetObjectPositionSequenceRequest{Requests: make([]*viewer.SetObjectPositionRequest, 100)}
			for j := range reqs.Requests {
				reqs.Requests[j] = makeRandomSetObjectPositionRequest(ids[j+i])
			}
			resp, err := client.SetObjectPositionSequence(ctx, reqs)
			if err != nil {
				log.Fatalf("Position RPC failed: %v", err)
			}
			for _, r := range resp.Responses {
				log.Printf("Position Response: %v", r)
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
