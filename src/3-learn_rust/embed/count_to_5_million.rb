# This program creates 10 threads that count to 5000000,
# and then prints "done!" when all threads are done.
# Due to GIL (global interpreter lock) in Ruby,
# only one core will be used in this program.
threads = []

10.times do
    threads << Thread.new do
        count = 0

        5_000_000.times do
            count += 1
        end

        count
    end
end

threads.each do |t|
    puts  "Thread finished with count=#{t.value}"
end
puts "done!"
