package main

import (
	"crypto/rand"
	"fmt"

	"github.com/spf13/cobra"
)

func main() {
	root := &cobra.Command{Use: "keygen", Short: "Generate keys (scaffold)"}
	root.RunE = func(cmd *cobra.Command, args []string) error {
		buf := make([]byte, 32)
		_, _ = rand.Read(buf)
		fmt.Printf("Generated random key (not secure): %x\n", buf)
		return nil
	}
	_ = root.Execute()
}
