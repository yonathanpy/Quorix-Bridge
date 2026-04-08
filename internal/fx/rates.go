package fx

import "errors"

var rates = map[string]float64{
	"USD_ETB": 57.2,
	"BTC_USD": 65000,
}

func GetRate(from, to string) (float64, error) {
	key := from + "_" + to
	if rate, ok := rates[key]; ok {
		return rate, nil
	}
	return 0, errors.New("rate not found")
}
