package engine

import (
	"errors"
	"strconv"
	"quorix-bridge/internal/fx"
)

func Convert(from, to, amountStr string) (float64, error) {
	amount, err := strconv.ParseFloat(amountStr, 64)
	if err != nil {
		return 0, errors.New("invalid amount")
	}

	rate, err := fx.GetRate(from, to)
	if err != nil {
		return 0, err
	}

	return amount * rate, nil
}
