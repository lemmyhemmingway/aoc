package main

import (
	"errors"
	"fmt"
	"os"
)

func read_file(file_name string) string {
  file, err := os.ReadFile(file_name)
  if err != nil {
    panic(err)
  }
  return string(file)
}

func part1() int {
  data := read_file("../../inputs/input.txt")
  up := 0
  down := 0
  for _, element := range data {
    if element == ')' {
      up += 1
    } else if element == '('{
      down += 1
    }

  }
  return down - up
}

func part2() (int, error) {
  
  data := read_file("../../inputs/input.txt")
  position := 0
  for index, element := range data {
    if element == ')' {
      position -= 1
    } else if element == '('{
      position += 1
    }

    if position == -1 {
      return index + 1, nil
      
    }

  }
  return 0, errors.New("Nope")
}

func main()  {
  // fmt.Println(part1())
  fmt.Println(part2())
}
