Rails.application.routes.draw do
  get 'number' => "rust_number#show_number"
  get 'compression_rating' => "rust_rating#show_compression"
end
