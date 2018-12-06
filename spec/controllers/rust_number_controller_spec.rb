require 'rails_helper'

RSpec.describe RustNumberController, type: :controller do

  describe "GET #show_number" do
    it "returns http success" do
      get :show_number
      expect(response).to have_http_status(:success)
    end
  end

end
