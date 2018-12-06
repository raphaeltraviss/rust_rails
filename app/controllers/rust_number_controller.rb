class RustNumberController < ApplicationController
  def show_number
    @number = SubjectDetect.get_number()
    render 'show_number'
  end
end
