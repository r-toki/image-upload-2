import Axios from 'axios';

const baseURL = import.meta.env.VITE_APP_BACKEND_URL;

export const axios = Axios.create({ baseURL });
