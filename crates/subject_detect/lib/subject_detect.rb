require "helix_runtime"

begin
  require "subject_detect/native"
rescue LoadError
  warn "Unable to load subject_detect/native. Please run `rake build`"
end
