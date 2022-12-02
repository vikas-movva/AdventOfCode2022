# get data from text file
with open('day1_input.txt', 'r') as f:
  elves = []
  # read first line
  line = f.readline().strip()
  
  # if line is not empty keep reading
  while line:
    
    # reset temp list
    temp = []
    
    # add each number to temp list until we reach a lone \n character
    while line != '':
      # add number to temp list
      temp.append(int(line))
      
      # read next line
      line = f.readline().strip()
    
    # add temp list to elves list
    elves.append(temp)
    
    # read next line
    line = f.readline().strip()

# create a list that has the length of the number of elves
nums = [0] * len(elves)

# loop through each elf
for i, elf in enumerate(elves):

  # assign the sum of the elf's numbers to the corresponding index in nums
  nums[i-1] = sum(elf)

# sort nums in descending order  
nums.sort(reverse=True)

# q2 ------------------------------------------------
# get the sum of the top 3 numbers
cals = nums[0] + nums[1] + nums[2]

# print the sum
print(cals)
  
