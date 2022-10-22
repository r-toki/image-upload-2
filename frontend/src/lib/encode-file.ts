export const fromFile = (file: File): Promise<{ encodedBytes: string; contentType: string }> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => {
      const result = reader.result as string;
      const [contentType, encodedBytes] = result.split('data:').join('').split(';base64,');
      resolve({ encodedBytes, contentType });
    };
    reader.onerror = (error) => reject(error);
  });
};
