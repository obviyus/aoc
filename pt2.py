def part_2():
    count, window = 0, []

    with open('input', 'r') as file:
        try:
            while (line := int(file.readline().rstrip())):
                if len(window) < 3:
                    window.append(line)
                else:
                    sum1 = sum(window)
                    
                    window.pop(0)
                    window.append(line)
                    sum2 = sum(window)
                    if sum2 > sum1:
                        count += 1
        except(ValueError):
            pass
    return count

if __name__ == '__main__':
    import timeit
    print(timeit.timeit("part_2()", globals=locals(), number=1000))