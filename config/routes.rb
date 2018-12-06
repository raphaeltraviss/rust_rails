Rails.application.routes.draw do
  get 'number' => "rust_number#show_number"
end
