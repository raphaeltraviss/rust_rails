require 'rails_helper'

RSpec.describe "posts/edit", type: :view do
  before(:each) do
    @post = assign(:post, Post.create!(
      :image_data => "MyText",
      :title => "MyText",
      :body => "MyText"
    ))
  end

  it "renders the edit post form" do
    render

    assert_select "form[action=?][method=?]", post_path(@post), "post" do

      assert_select "textarea[name=?]", "post[image_data]"

      assert_select "textarea[name=?]", "post[title]"

      assert_select "textarea[name=?]", "post[body]"
    end
  end
end
