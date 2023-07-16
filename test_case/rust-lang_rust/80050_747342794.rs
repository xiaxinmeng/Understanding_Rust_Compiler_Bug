ruby
File.open('src/lib.rs', 'w') do |file|
  0..10_000.times do |n|
      file.write("pub struct MyType#{n} { pub field: i32 }\n")
  end
end
