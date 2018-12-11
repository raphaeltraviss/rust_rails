class Post < ApplicationRecord
  include ImageUploader[:image]
  validates :body, presence: true
end
