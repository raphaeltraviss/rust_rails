class RustRatingController < ApplicationController
  def show_compression
    @number = SubjectDetect.get_compression_rating()
    render 'rust_number/show_number'
  end
end
