package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	root := &cobra.Command{Use: "blockswap", Short: "BlockSwap CLI (scaffold)"}

	root.AddCommand(cmdInit(), cmdStart())

	if err := root.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func cmdInit() *cobra.Command {
	return &cobra.Command{
		Use:   "init",
		Short: "Initialize node configuration",
		RunE: func(cmd *cobra.Command, args []string) error {
			fmt.Println("Initialized node (stub)")
			return nil
		},
	}
}

func cmdStart() *cobra.Command {
	return &cobra.Command{
		Use:   "start",
		Short: "Start node",
		RunE: func(cmd *cobra.Command, args []string) error {
			fmt.Println("Starting node (stub)")
			return nil
		},
	}
}
