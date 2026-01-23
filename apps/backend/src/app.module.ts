import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TestExceptionController } from './test-exception.controller';

@Module({
  imports: [],
  controllers: [AppController, TestExceptionController],
  providers: [AppService],
})
export class AppModule {}
