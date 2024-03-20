num_of_lines = 0
number_of_words = [] 
sum = 0
File.open(ARGV[0]).each do |line|
    number_of_words.push(line.length)
    puts line
    num_of_lines += 1
end
 number_of_words.each { |a| sum+=a }
 puts sum