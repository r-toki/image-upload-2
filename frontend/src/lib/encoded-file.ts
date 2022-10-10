export class EncodedFile {
  constructor(public contentType: string, public encodedBytes: string) {}

  get toString() {
    return ['data:', this.contentType, ';base64,', this.encodedBytes].join('');
  }

  private static fromString(str: string) {
    str = str.split('data:').join('');
    const [contentType, encodedBytes] = str.split(';base64,');
    return new EncodedFile(contentType, encodedBytes);
  }

  static fromFile(file: File): Promise<EncodedFile> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsDataURL(file);
      reader.onload = () => {
        const result = reader.result as string;
        resolve(EncodedFile.fromString(result));
      };
      reader.onerror = (error) => reject(error);
    });
  }
}
