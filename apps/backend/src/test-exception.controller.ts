import { Controller, Get, HttpException, HttpStatus } from '@nestjs/common';

@Controller('test-exception')
export class TestExceptionController {
  @Get('http-exception')
  getHttpException() {
    throw new HttpException('Test HTTP exception message', HttpStatus.BAD_REQUEST);
  }

  @Get('general-error')
  getGeneralError() {
    throw new Error('Test general error message');
  }

  @Get('internal-server-error')
  getInternalServerError() {
    // This will trigger the unknown error path
    throw 'Unknown error type'; // Throwing a string to simulate unknown error type
  }
}